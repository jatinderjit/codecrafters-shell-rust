use std::{path::PathBuf, str::FromStr};

use crate::{builtins::Builtin, env_path::find_binary};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseExecutableError;

pub(crate) enum Executable {
    Builtin(Builtin),
    Binary(PathBuf),
}

impl FromStr for Executable {
    type Err = ParseExecutableError;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        if let Ok(builtin) = command.parse::<Builtin>() {
            return Ok(Executable::Builtin(builtin));
        }
        match find_binary(command) {
            Some(path) => Ok(Executable::Binary(path)),
            None => Err(ParseExecutableError),
        }
    }
}
