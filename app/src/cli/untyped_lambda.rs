use super::{display_or_debug, Source};
use common::{Eval, Parse};
use std::error::Error;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
    /// use debug print instead of regular
    #[clap(short, long)]
    debug: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let source = args.source.get_source()?;
    let parsed = untyped_lambda::terms::Term::parse(source)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("parsed: {parsed_str}");
    }
    let evaled = parsed.eval_start().unwrap();
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("evaluated: {evaled_str}");
    Ok(())
}
