use clap::{Parser, Subcommand};
mod errors;
mod untyped_arithmetic;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    UntypedArithmetic(untyped_arithmetic::Args),
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::UntypedArithmetic(args) => {
            untyped_arithmetic::exec(args).map_err(|err| err.to_string())
        }
    }
}
