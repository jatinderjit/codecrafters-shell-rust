use std::{
    env::{self, current_dir, set_current_dir},
    str::FromStr,
};

use crate::{
    env_path::{expand_home, home_dir},
    executables::Executable,
};

pub(crate) enum Builtin {
    Cd,
    Echo,
    Exit,
    Pwd,
    Type,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseBuiltinError;

impl FromStr for Builtin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Builtin::*;

        match s {
            "cd" => Ok(Cd),
            "echo" => Ok(Echo),
            "exit" => Ok(Exit),
            "pwd" => Ok(Pwd),
            "type" => Ok(Type),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl Builtin {
    pub(crate) fn execute(&self, args: Vec<String>) {
        use Builtin::*;

        match self {
            Cd => process_cd(args),
            Echo => println!("{}", args.join(" ")),
            Exit => process_exit(args),
            Pwd => println!("{}", env::current_dir().unwrap().display()),
            Type => process_type(args),
        }
    }
}

fn process_cd(args: Vec<String>) {
    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }
    let mut path = match args.first() {
        Some(path) => expand_home(path),
        None => home_dir(),
    };
    if path.is_relative() {
        path = current_dir().unwrap().join(&path);
    }
    if path.is_dir() {
        set_current_dir(path).unwrap();
    } else {
        println!("cd: {}: No such file or directory", args[0]);
    }
}

fn process_exit(args: Vec<String>) {
    if args.len() > 1 {
        println!("exit: too many arguments");
        return;
    }
    let code = match args.first() {
        Some(code) => code.parse().unwrap_or(1),
        None => 0,
    };
    std::process::exit(code);
}

fn process_type(args: Vec<String>) {
    for subcommand in args {
        match subcommand.parse::<Executable>() {
            Ok(Executable::Builtin(_)) => println!("{subcommand} is a shell builtin"),
            Ok(Executable::Binary(path)) => println!("{subcommand} is {}", path.display()),
            Err(_) => println!("{subcommand}: not found"),
        };
    }
}
