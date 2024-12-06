use std::str::FromStr;

pub(crate) enum Builtin {
    Echo,
    Exit,
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
            _ => Err(ParseBuiltinError),
        }
    }
}

impl Builtin {
    pub(crate) fn execute(&self, args: Option<&str>) {
        use Builtin::*;

        match self {
            Exit => process_exit(args),
            Echo => println!("{}", args.unwrap_or("")),
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
