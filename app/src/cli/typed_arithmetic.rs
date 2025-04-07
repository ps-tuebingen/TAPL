use super::{display_or_debug, Source};
use std::error::Error;
use typed_arithmetic::{check::check, eval::eval, parser::parse};

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
        println!("parsed: {parsed_str}")
    }
    let checked = check(&parsed)?;
    if args.verbose {
        let checked_str = display_or_debug(&checked, args.debug);
        println!("checked: {checked_str}");
    }
    let evaled = eval(parsed)?;
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("evaled: {evaled_str}");
    Ok(())
}
