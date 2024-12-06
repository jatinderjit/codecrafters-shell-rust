use std::{
    env::{self, current_dir, set_current_dir},
    str::FromStr,
};

use crate::executables::Executable;

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
    pub(crate) fn execute(&self, args: Option<&str>) {
        use Builtin::*;

        match self {
            Cd => process_cd(args),
            Echo => println!("{}", args.unwrap_or("")),
            Exit => process_exit(args),
            Pwd => println!("{}", env::current_dir().unwrap().display()),
            Type => process_type(args),
        }
    }
}

#[allow(deprecated)]
fn process_cd(args: Option<&str>) {
    let mut path = match args {
        Some(path) => path.into(),
        None => env::home_dir().unwrap(),
    };
    if path.starts_with("~") {
        path = path.strip_prefix("~").unwrap().to_owned();
        path = env::home_dir().unwrap().join(path);
    } else if path.is_relative() {
        path = current_dir().unwrap().join(path);
    }
    set_current_dir(path).unwrap();
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
    match subcommand.parse::<Executable>() {
        Ok(Executable::Builtin(_)) => println!("{subcommand} is a shell builtin"),
        Ok(Executable::Binary(path)) => println!("{subcommand} is {}", path.display()),
        Err(_) => println!("{subcommand}: not found"),
    };
}
