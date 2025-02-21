use super::errors::Error;
use std::{fs::read_to_string, path::PathBuf};
use untyped_arithmetic::parse::parse;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
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

pub fn exec(args: Args) -> Result<(), Error> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        println!("Successfully parsed {parsed:?}\n");
    }
    let evaled = parsed.eval();
    println!("{evaled:?}");
    Ok(())
}
