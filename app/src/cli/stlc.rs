use super::{errors::Error, Source};
use stlc::parser::parse;

#[derive(clap::Args)]
pub struct Args {
    #[clap(flatten)]
    source: Source,
    /// Print additional information
    #[clap(short, long)]
    verbose: bool,
}

pub fn exec(args: Args) -> Result<(), Error> {
    let src = args.source.get_source()?;
    let parsed = parse(src);
    /*if args.verbose {
        println!("Successfully parsed {parsed:?}\n");
    }*/
    //let evaled = parsed.eval();
    //println!("{evaled:?}");
    Ok(())
}
