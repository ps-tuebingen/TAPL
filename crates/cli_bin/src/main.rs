use clap::Parser;
use std::fmt;

pub mod args;
use args::{Args, Command};

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
    let method = args.out_method.to_format_method();
    let res = match args.cmd {
        Command::Parse => args.lang.parse(src, &method),
        Command::Check => args.lang.check(src, &method),
        Command::Evaluate => args.lang.eval(src, &method),
    };
    match res {
        Ok(res) => println!("{res}"),
        Err(err) => println!("Error during {}:\n\t{err}", args.cmd),
    }
}
