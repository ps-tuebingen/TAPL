use clap::Parser;

mod cli;

use cli::{untyped_arithmetic, Cli, Command};

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::UntypedArithmetic(args) => {
            untyped_arithmetic::exec(args).map_err(|err| err.to_string())
        }
        Command::UntypedLambda(args) => todo!(),
    }
}
