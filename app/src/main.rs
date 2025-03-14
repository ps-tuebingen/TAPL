use clap::Parser;

mod cli;

use cli::{nameless_representation, stlc, untyped_arithmetic, untyped_lambda, Cli, Command};

fn main() -> Result<(), String> {
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
    }
}
