use clap::{Parser, Subcommand};
use std::{error::Error, fmt, fs::read_to_string, path::PathBuf};

pub mod bounded_quantification;
pub mod exceptions;
pub mod featherweight;
pub mod inference;
pub mod lambda_omega;
pub mod nameless_representation;
pub mod recursive;
pub mod references;
pub mod stlc;
pub mod subtypes;
pub mod system_f;
pub mod untyped_arithmetic;
pub mod untyped_lambda;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    UntypedArithmetic(untyped_arithmetic::Args),
    UntypedLambda(untyped_lambda::Args),
    NamelessRepresentation(nameless_representation::Args),
    Stlc(stlc::Args),
    References(references::Args),
    Exceptions(exceptions::Args),
    Subtypes(subtypes::Args),
    Featherweight(featherweight::Args),
    Recursive(recursive::Args),
    Inference(inference::Args),
    SystemF(system_f::Args),
    BoundedQuantification(bounded_quantification::Args),
    LambdaOmega(lambda_omega::Args),
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
    fn get_source(self) -> Result<String, Box<dyn Error>> {
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

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::UntypedArithmetic(args) => {
            untyped_arithmetic::exec(args).map_err(|err| err.to_string())
        }
        Command::UntypedLambda(args) => untyped_lambda::exec(args).map_err(|err| err.to_string()),
        Command::NamelessRepresentation(args) => {
            nameless_representation::exec(args).map_err(|err| err.to_string())
        }
        Command::Stlc(args) => stlc::exec(args).map_err(|err| err.to_string()),
        Command::References(args) => references::exec(args).map_err(|err| err.to_string()),
        Command::Exceptions(args) => exceptions::exec(args).map_err(|err| err.to_string()),
        Command::Subtypes(args) => subtypes::exec(args).map_err(|err| err.to_string()),
        Command::Featherweight(args) => featherweight::exec(args).map_err(|err| err.to_string()),
        Command::Recursive(args) => recursive::exec(args).map_err(|err| err.to_string()),
        Command::Inference(args) => inference::exec(args).map_err(|err| err.to_string()),
        Command::SystemF(args) => system_f::exec(args).map_err(|err| err.to_string()),
        Command::BoundedQuantification(args) => {
            bounded_quantification::exec(args).map_err(|err| err.to_string())
        }
        Command::LambdaOmega(args) => lambda_omega::exec(args).map_err(|err| err.to_string()),
    }
}

pub fn display_or_debug<T: fmt::Debug + fmt::Display>(t: &T, debug: bool) -> String {
    if debug {
        format!("{t:?}")
    } else {
        format!("{t}")
    }
}
