#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        repl();
    }
}

fn repl() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let command = input.trim_matches(['\r', '\n']);
    process(command);
}

fn process(command: &str) {
    let command_with_args = command.splitn(2, ' ').collect::<Vec<_>>();

    match command_with_args[0] {
        "exit" => exit(&command_with_args[1..]),
        "echo" => println!("{}", command_with_args.get(1).unwrap_or(&"")),
        _ => println!("{command}: command not found"),
    };
}

fn exit(args: &[&str]) -> ! {
    if args.len() == 1 {
        let code = args[0].parse::<u8>().unwrap_or(1);
        std::process::exit(code as i32);
    }
    std::process::exit(0);
}
