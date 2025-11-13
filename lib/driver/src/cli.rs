use crate::format::FormatMethod;
use clap::Parser;
use errors::{FileAccess, driver_error::DriverError};
use languages::AllLanguages;
use std::{fmt, fs::read_to_string, path::PathBuf, str::FromStr};

#[derive(Parser)]
pub struct Args {
    pub lang: AllLanguages,
    pub cmd: Command,
    pub out_method: Option<FormatMethod>,
    #[clap(flatten)]
    pub source: Source,
    #[clap(short, long)]
    pub out_file: Option<PathBuf>,
}

impl Args {
    #[must_use]
    pub fn method(&self) -> FormatMethod {
        self.out_method.unwrap_or(FormatMethod::Simple)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Parse,
    Evaluate,
    Check,
    Grammar,
}

impl FromStr for Command {
    type Err = DriverError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "parse" => Ok(Self::Parse),
            "eval" | "evaluate" => Ok(Self::Evaluate),
            "check" | "typecheck" => Ok(Self::Check),
            "grammar" => Ok(Self::Grammar),
            _ => Err(DriverError::UndefinedCommand(
                "Not a valid command".to_owned(),
            )),
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

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = false)]
pub struct Source {
    /// Load source from file
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// load source from command line argument
    #[clap(short, long)]
    input: Option<String>,
}

impl Source {
    /// Get the source string specified in the command line
    /// either the passed literal string or the read file
    /// # Errors
    /// Returns an error if either no source was provided or the file could not be read
    pub fn get_source(&self) -> Result<String, DriverError> {
        if let Some(ref src) = self.input {
            return Ok(src.clone());
        }

        if let Some(ref path) = self.file {
            let contents =
                read_to_string(path).map_err(|err| FileAccess::new("read source file", err))?;
            return Ok(contents);
        }

        Err(DriverError::EmptyInput)
    }
}
