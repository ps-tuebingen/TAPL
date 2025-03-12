use clap::{Parser, Subcommand};
use std::{fs::read_to_string, path::PathBuf};

mod errors;
pub mod stlc;
pub mod untyped_arithmetic;
pub mod untyped_lambda;

use errors::Error;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    UntypedArithmetic(untyped_arithmetic::Args),
    UntypedLambda(untyped_lambda::Args),
    Stlc(stlc::Args),
}

#[derive(Debug, clap::Args)]
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
    fn get_source(self) -> Result<String, Error> {
        if let Some(src) = self.input {
            return Ok(src);
        }

        if let Some(path) = self.file {
            let contents = read_to_string(path)?;
            return Ok(contents);
        }

        panic!("Either --file or --input must be provided")
    }
}
