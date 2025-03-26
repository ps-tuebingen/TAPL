use super::{display_or_debug, Error, Source};
use system_f::{check::Check, eval::Eval, parser::parse};

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

pub fn exec(args: Args) -> Result<(), Error> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("Parsed: {parsed_str}");
    }

    let checked = parsed.clone().check(&mut Default::default())?;
    if args.verbose {
        let checked_str = display_or_debug(&checked, args.debug);
        println!("checked: {checked_str}");
    }

    let evaled = parsed.eval()?;
    let evaled_str = display_or_debug(&evaled, args.debug);
    println!("evaluated: {evaled_str}");
    Ok(())
}
