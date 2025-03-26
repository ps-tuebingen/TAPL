use super::{display_or_debug, Source};
use featherweight::{parser::parse, typing::class::class_ok};
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
    for class in parsed.classes.clone().into_values() {
        let name = class.name.clone();
        if args.verbose {
            let class_str = display_or_debug(&class, args.debug);
            println!("{class_str}");
        }
        let checked = class_ok(class, &parsed);
        if !checked {
            println!("class {name} could not be checked");
            return Ok(());
        } else if args.verbose {
            println!("checked class {name}");
        }
    }
    println!("All classes checked successfully");
    Ok(())
}
