use super::{display_or_debug, Source};
use references::{check::check, eval::eval, parser::parse};
use std::error::Error;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
    /// use debug print intead of regular
    #[clap(short, long)]
    debug: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("parsed: {parsed_str}");
    }
    let checked = check(
        parsed.clone(),
        &mut Default::default(),
        &mut Default::default(),
    )?;
    if args.verbose {
        let checked_str = display_or_debug(&checked, args.debug);
        println!("Type Checked: {checked_str}");
    }

    let evaled = eval(parsed, &mut Default::default())?;
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("Evaled: {evaled_str}");
    Ok(())
}
