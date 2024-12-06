use std::str::FromStr;

use crate::env_path::find_binary;

pub(crate) enum Builtin {
    Echo,
    Exit,
    Type,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseBuiltinError;

impl FromStr for Builtin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Builtin::*;

        match s {
            "echo" => Ok(Echo),
            "exit" => Ok(Exit),
            "type" => Ok(Type),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl Builtin {
    pub(crate) fn execute(&self, args: Option<&str>) {
        use Builtin::*;

        match self {
            Echo => println!("{}", args.unwrap_or("")),
            Exit => process_exit(args),
            Type => process_type(args),
        }
    }
}

fn process_exit(args: Option<&str>) -> ! {
    let code = match args {
        Some(code) => code.parse().unwrap_or(1),
        None => 0,
    };
    std::process::exit(code);
}

fn process_type(args: Option<&str>) {
    let subcommand = match args {
        Some(subcommand) => subcommand,
        None => return,
    };
    match subcommand.parse::<Builtin>() {
        Ok(_) => println!("{subcommand} is a shell builtin"),
        Err(_) => match find_binary(subcommand) {
            Some(path) => println!("{subcommand} is {}", path.to_string_lossy()),
            None => println!("{subcommand}: not found"),
        },
    };
}
