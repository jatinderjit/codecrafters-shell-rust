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

fn parse_command(input: &str) -> Result<(Option<String>, Vec<String>), &'static str> {
    let mut args = match parse_args(input) {
        Ok(args) => args,
        Err(err) => return Err(err),
    };
    let command = if args.is_empty() {
        None
    } else {
        Some(args.remove(0))
    };
    Ok((command, args))
}

fn parse_args(input: &str) -> Result<Vec<String>, &'static str> {
    let mut args = vec![];
    let mut it = input.chars().peekable();
    skip_whitespace(&mut it);
    let mut whitespace_skipped = true;
    while it.peek().is_some() {
        match parse_arg(&mut it) {
            Ok(arg) => {
                if whitespace_skipped {
                    args.push(arg)
                } else {
                    // There was no whitespace between the last argument and the
                    // new argument. Append this to the last argument.
                    let last = args.len() - 1;
                    args[last].extend(arg);
                }
            }
            Err(err) => return Err(err),
        }
        whitespace_skipped = skip_whitespace(&mut it);
    }
    Ok(args
        .into_iter()
        .map(|arg| arg.into_iter().collect())
        .collect())
}

fn parse_arg(it: &mut Peekable<Chars>) -> Result<Vec<char>, &'static str> {
    match it.peek().copied() {
        Some('\'') => parse_arg_in_single_quotes(it),
        Some('"') => parse_arg_in_double_quotes(it),
        Some(_) => parse_naked_arg(it),
        None => Ok(Vec::new()),
    }
}

fn parse_naked_arg(it: &mut Peekable<Chars>) -> Result<Vec<char>, &'static str> {
    // Argument not wrapped in quotes.
    let mut arg = vec![];
    while let Some(ch) = it.peek().copied() {
        if ch == ' ' || ch == '\t' {
            break;
        }
        it.next();
        if ch == '\\' {
            let next_ch = match it.next() {
                Some(ch) => ch,
                None => return Err("No character to escape"),
            };
            arg.push(next_ch);
        } else {
            arg.push(ch);
        }
    }
    Ok(arg)
}

fn parse_arg_in_single_quotes(it: &mut Peekable<Chars>) -> Result<Vec<char>, &'static str> {
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
    Ok(arg)
}

fn parse_arg_in_double_quotes(it: &mut Peekable<Chars>) -> Result<Vec<char>, &'static str> {
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
    Ok(arg)
}

fn skip_whitespace(it: &mut Peekable<Chars>) -> bool {
    let mut skipped = false;
    while let Some(ch) = it.peek().copied() {
        if ch == ' ' || ch == '\t' {
            it.next();
            skipped = true;
        } else {
            break;
        }
    }
    skipped
}
