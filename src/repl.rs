use std::{
    io::{self, Write},
    iter::Peekable,
    path::PathBuf,
    process::Command,
    str::Chars,
};

use crate::executables::Executable;

pub fn run() {
    loop {
        let input = read_line("$ ");

        execute(input.trim());
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}

fn execute(input: &str) {
    let (command, args) = match parse_command(input) {
        Ok((command, args)) => (command, args),
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    let command = match command {
        Some(command) if !command.is_empty() => command,
        _ => return,
    };

    let executable = command.parse::<Executable>();
    match executable {
        Ok(Executable::Builtin(builtin)) => builtin.execute(args),
        Ok(Executable::Binary(path)) => execute_binary(path, args),
        Err(_) => println!("{command}: command not found"),
    };
}

fn execute_binary(path: PathBuf, args: Vec<String>) {
    let mut command = Command::new(path);
    command.args(args);
    command.status().expect("Failed to execute process");
}

fn parse_command(input: &str) -> Result<(Option<&str>, Vec<String>), &'static str> {
    let mut command_with_args = input.splitn(2, ' ');
    let command = command_with_args.next();
    let args = match command_with_args.next() {
        Some(input) => match parse_args(input) {
            Ok(args) => args,
            Err(err) => return Err(err),
        },
        None => Vec::new(),
    };
    Ok((command, args))
}

fn parse_args(input: &str) -> Result<Vec<String>, &'static str> {
    let mut args = vec![];
    let mut it = input.chars().peekable();
    skip_whitespace(&mut it);
    while it.peek().is_some() {
        match parse_arg(&mut it) {
            Ok(arg) => args.push(arg),
            Err(err) => return Err(err),
        }
        skip_whitespace(&mut it);
    }
    Ok(args)
}

fn parse_arg(it: &mut Peekable<Chars>) -> Result<String, &'static str> {
    match it.peek().copied() {
        Some(ch) if ch == '\'' => return parse_arg_in_single_quotes(it),
        Some(ch) if ch == '"' => return parse_arg_in_double_quotes(it),
        Some(_) => {}
        None => return Ok(String::new()),
    };

    // Argument not wrapped in quotes.
    let mut arg = vec![];
    for ch in it {
        if ch == ' ' || ch == '\t' {
            break;
        }
        arg.push(ch);
    }
    Ok(arg.into_iter().collect())
}

fn parse_arg_in_single_quotes(it: &mut Peekable<Chars>) -> Result<String, &'static str> {
    it.next();
    let mut arg = vec![];
    let mut closed = false;
    for ch in it {
        if ch == '\'' {
            closed = true;
            break;
        }
        arg.push(ch);
    }
    if !closed {
        return Err("Invalid arguments: quotes not closed");
    }
    Ok(arg.into_iter().collect())
}

fn parse_arg_in_double_quotes(it: &mut Peekable<Chars>) -> Result<String, &'static str> {
    it.next();
    let mut arg = vec![];
    let mut closed = false;
    while let Some(ch) = it.next() {
        if ch == '"' {
            closed = true;
            break;
        }
        if ch == '\\' {
            match it.peek().copied() {
                Some(next_ch) if ['$', '`', '"', '\\'].contains(&next_ch) => {
                    it.next();
                    arg.push(next_ch);
                }
                _ => arg.push(ch),
            };
        } else {
            arg.push(ch);
        }
    }
    if !closed {
        return Err("Invalid arguments: quotes not closed");
    }
    Ok(arg.into_iter().collect())
}

fn skip_whitespace(it: &mut Peekable<Chars>) {
    while let Some(ch) = it.peek().copied() {
        if ch == ' ' || ch == '\t' {
            it.next();
        } else {
            break;
        }
    }
}
