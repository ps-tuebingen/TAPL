use errors::language_error::LanguageError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Parse,
    Evaluate,
    Check,
    GenerateConstraints,
    SolveConstraints,
    Infer,
    Grammar,
}

impl Command {
    /// Array of all commands
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::Parse, Self::Evaluate, Self::Check, Self::Grammar]
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
            "generate" | "generate-constraints" => Ok(Self::GenerateConstraints),
            "solve" | "solve-constraints" => Ok(Self::SolveConstraints),
            "infer" | "inference" => Ok(Self::Infer),
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
            Self::GenerateConstraints => f.write_str("generate"),
            Self::SolveConstraints => f.write_str("solve"),
            Self::Infer => f.write_str("infer"),
        }
    }
}
