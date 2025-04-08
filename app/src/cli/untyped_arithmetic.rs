use super::{display_or_debug, Source};
use common::Eval;
use std::error::Error;
use untyped_arithmetic::parse::parse;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
    /// Use debug print instead of regular
    #[clap(short, long)]
    debug: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("Successfully parsed {parsed_str}\n");
    }
    let evaled = parsed.eval(&mut Default::default())?;
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("{evaled_str}");
    Ok(())
}
