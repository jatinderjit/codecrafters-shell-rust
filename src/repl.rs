use std::{
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::executables::Executable;

pub fn run() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim_matches(['\r', '\n']);
        process(input);
    }
}

fn process(input: &str) {
    let command_with_args = input.splitn(2, ' ').collect::<Vec<_>>();
    let command = command_with_args[0];
    let args = command_with_args.get(1).copied();

    let executable = command.parse::<Executable>();
    match executable {
        Ok(Executable::Builtin(builtin)) => builtin.execute(args),
        Ok(Executable::Binary(path)) => execute_binary(path, args),
        Err(_) => println!("{command}: command not found"),
    };
}

fn execute_binary(path: PathBuf, args: Option<&str>) {
    let mut command = Command::new(path);
    if let Some(args) = args {
        command.arg(args);
    }
    command.status().expect("Failed to execute process");
}
