use errors::language_error::LanguageError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Parse,
    Evaluate,
    Check,
    Grammar,
}

impl Command {
    pub const fn all() -> [Command; 4] {
        [
            Command::Parse,
            Command::Evaluate,
            Command::Check,
            Command::Grammar,
        ]
    }
}

impl FromStr for Command {
    type Err = LanguageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "parse" => Ok(Self::Parse),
            "eval" | "evaluate" => Ok(Self::Evaluate),
            "check" | "typecheck" => Ok(Self::Check),
            "grammar" => Ok(Self::Grammar),
            _ => Err(LanguageError::UndefinedCommand(s.to_string())),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse => f.write_str("parse"),
            Self::Evaluate => f.write_str("evaluate"),
            Self::Check => f.write_str("check"),
            Self::Grammar => f.write_str("grammar"),
        }
    }
}
