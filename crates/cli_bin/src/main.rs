use clap::Parser;
use std::{fs::File, io::Write, path::PathBuf};

pub mod args;
use args::{Args, Command};

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
        Ok(res) => write_result(res, args.out_file).unwrap(),
        Err(err) => panic!("Error during {}:\n\t{err}", args.cmd),
    }
}

fn write_result(res: String, out_path: Option<PathBuf>) -> Result<(), String> {
    if let Some(path) = out_path {
        let mut file = File::create(path).map_err(|err| err.to_string())?;
        file.write_all(res.as_bytes())
            .map_err(|err| err.to_string())?;
        Ok(())
    } else {
        println!("{res}");
        Ok(())
    }
}
