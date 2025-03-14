use super::{errors::Error, Source};
use stlc::{check::Check, eval::Eval, eval_context::eval_with_context, parser::parse};

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
    /// use evaluation contexts
    #[clap(short, long)]
    eval_context: bool,
}

pub fn exec(args: Args) -> Result<(), Error> {
    let src = args.source.get_source()?;
    let parsed = parse(src)?;
    if args.verbose {
        println!("parsed term: {parsed}");
    }
    let checked = parsed.check(&mut Default::default())?;
    if args.verbose {
        println!("type checked: {checked}")
    }
    if args.eval_context {
        let evaled = eval_with_context(parsed)?;
        println!("evaluated: {evaled}");
    } else {
        let evaled = parsed.eval()?;
        println!("evaluated: {evaled}");
    }
    Ok(())
}
