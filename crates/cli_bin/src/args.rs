use clap::Parser;
use language::{AllLanguages, FormatMethod};
use std::fmt;
use std::{convert::Infallible, fs::read_to_string, path::PathBuf, str::FromStr};

#[derive(Parser)]
pub struct Args {
    pub lang: AllLanguages,
    pub cmd: Command,
    #[clap(flatten)]
    pub source: Source,
    #[clap(flatten)]
    pub out_method: OutMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Parse,
    Evaluate,
    Check,
}

impl FromStr for Command {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "parse" => Ok(Command::Parse),
            "eval" | "evaluate" => Ok(Command::Evaluate),
            "check" | "typecheck" => Ok(Command::Check),
            _ => panic!("Not a valid command"),
        }
    }
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Parse => f.write_str("parse"),
            Command::Evaluate => f.write_str("evaluate"),
            Command::Check => f.write_str("check"),
        }
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Source {
    /// Load source from file
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// load source from command line argument
    #[clap(short, long)]
    input: Option<String>,
}

impl Source {
    pub fn get_source(&self) -> String {
        if let Some(ref src) = self.input {
            return src.clone();
        }

        if let Some(ref path) = self.file {
            let contents = read_to_string(path).expect("Could not read input file");
            return contents;
        }

        panic!("Either --file or --input must be provided")
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = false)]
pub struct OutMethod {
    #[clap(short, long)]
    latex: bool,
    #[clap(short, long)]
    dbg: bool,
}

impl OutMethod {
    pub fn to_format_method(&self) -> FormatMethod {
        if self.latex {
            FormatMethod::Latex
        } else if self.dbg {
            FormatMethod::Debug
        } else {
            FormatMethod::Simple
        }
    }
}
