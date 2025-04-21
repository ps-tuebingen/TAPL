use super::Source;
use common::Parse;
use nameless_representation::{remove_names::remove_names, restore_names::restore_names};
use std::error::Error;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additonal information
    #[clap(short, long)]
    verbose: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let src = args.source.get_source()?;
    let parsed = untyped_lambda::terms::Term::parse(src)?;
    if args.verbose {
        println!("parsed: {parsed}");
    }
    let removed = remove_names(parsed.into());
    if args.verbose {
        println!("removed names: {removed}");
    }
    let restored = restore_names(removed);
    if args.verbose {
        println!("restored names: {restored}");
    }
    Ok(())
}
