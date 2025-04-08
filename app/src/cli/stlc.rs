use super::{display_or_debug, Source};
use common::Eval;
use std::error::Error;
use stlc::{check::Check, eval_context::eval_with_context, parser::parse};

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
    /// use evaluation contexts
    #[clap(short, long)]
    eval_context: bool,
}

pub fn exec(args: Args) -> Result<(), Box<dyn Error>> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        let parsed_str = display_or_debug(&parsed, args.debug);
        println!("parsed term: {parsed_str}");
    }
    let checked = parsed.check(&mut Default::default())?;
    if args.verbose {
        let checked_str = display_or_debug(&checked, args.debug);
        println!("type checked: {checked_str}")
    }
    if args.eval_context {
        let evaled = eval_with_context(parsed)?;
        let evaled_str = display_or_debug(&evaled, args.debug);
        println!("evaluated: {evaled_str}");
    } else {
        let evaled = parsed.eval(&mut Default::default())?;
        let evaled_str = display_or_debug(&evaled, args.debug);
        println!("evaluated: {evaled_str}");
    }
    Ok(())
}
