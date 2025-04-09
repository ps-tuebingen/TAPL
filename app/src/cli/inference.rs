use super::{display_or_debug, Source};
use common::Parse;
use inference::{bidirectional::Infer, constraints::typecheck};
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
    // use bidirectional inference instead of constraint-based
    #[clap(short, long)]
    bidirectional: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let src = args.source.get_source()?;
    let parsed = inference::syntax::Term::parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("Parsed: {parsed_str}");
    }
    let checked = if args.bidirectional {
        parsed.infer(&mut Default::default())?
    } else {
        typecheck(parsed)?
    };
    let checked_str = display_or_debug(&checked, args.debug);
    println!("checked: {checked_str}");

    Ok(())
}
