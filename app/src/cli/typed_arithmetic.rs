use super::{display_or_debug, Source};
use common::{check::Typecheck, Eval, Parse};
use std::error::Error;

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
    let parsed = typed_arithmetic::terms::Term::parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("parsed: {parsed_str}")
    }
    let checked = parsed.check_start()?;
    if args.verbose {
        let checked_str = display_or_debug(&checked, args.debug);
        println!("checked: {checked_str}");
    }
    let evaled = parsed.eval_start()?;
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("evaled: {evaled_str}");
    Ok(())
}
