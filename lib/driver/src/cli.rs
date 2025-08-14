use crate::{format::FormatMethod, languages::AllLanguages};
use clap::Parser;
use errors::driver_error::DriverError;
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
    pub fn method(&self) -> FormatMethod {
        match self.out_method {
            Some(mthd) => mthd,
            None => FormatMethod::Simple,
        }
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
            "parse" => Ok(Command::Parse),
            "eval" | "evaluate" => Ok(Command::Evaluate),
            "check" | "typecheck" => Ok(Command::Check),
            "grammar" => Ok(Command::Grammar),
            _ => Err(DriverError::UndefinedCommand(
                "Not a valid command".to_owned(),
            )),
        }
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Parse => f.write_str("parse"),
            Command::Evaluate => f.write_str("evaluate"),
            Command::Check => f.write_str("check"),
            Command::Grammar => f.write_str("grammar"),
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
    pub fn get_source(&self) -> Result<String, DriverError> {
        if let Some(ref src) = self.input {
            return Ok(src.clone());
        }

        if let Some(ref path) = self.file {
            let contents = read_to_string(path).expect("Could not read input file");
            return Ok(contents);
        }

        Err(DriverError::EmptyInput)
    }
}
