use clap::Parser;
use language::AllLanguages;
use std::{fmt, fs::read_to_string, path::PathBuf};

#[derive(Parser)]
pub struct Args {
    lang: AllLanguages,
    #[clap(flatten)]
    source: Source,
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    debug: bool,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Source {
    /// Load source from file
    #[clap(short, long)]
    file: Option<PathBuf>,
    /// load source from command line argument
    #[clap(short, long)]
    input: Option<String>,
}

impl Source {
    fn get_source(&self) -> String {
        if let Some(ref src) = self.input {
            return src.clone();
        }

        if let Some(ref path) = self.file {
            let contents = read_to_string(path).expect("Could not read input file");
            return contents;
        }

        panic!("Either --file or --input must be provided")
    }
}

pub fn display_or_debug<T: fmt::Debug + fmt::Display>(t: &T, debug: bool) -> String {
    if debug {
        format!("{t:?}")
    } else {
        format!("{t}")
    }
}

fn main() {
    let args = Args::parse();
    let src = args.source.get_source();
    args.lang.run(
        src,
        args.debug,
        |p| println!("parsed: {p}"),
        |ty| println!("checked: {ty}"),
        |v| println!("evaluated: {v}"),
        |err| println!("Program exited with error {err}"),
    );
}
