use std::io::{self, Write};

use crate::builtins::Builtin;

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

fn invalid_command(command: &str) {
    println!("{command}: command not found");
}

fn process(input: &str) {
    let command_with_args = input.splitn(2, ' ').collect::<Vec<_>>();
    let command = command_with_args[0];
    let args = command_with_args.get(1).copied();

    let builtin = command.parse::<Builtin>();
    match builtin {
        Ok(builtin) => builtin.execute(args),
        Err(_) => invalid_command(command),
    };
}
