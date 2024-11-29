use std::env;
use untyped_lambda::{
    eval::{eval, EvalOrder},
    parse::parse,
};

fn main() -> Result<(), String> {
    let mut args = env::args();
    //remove file name
    args.next();
    let mut arg_str = args.collect::<Vec<String>>().join(" ");
    let parsed = parse(&mut arg_str).map_err(|err| err.to_string())?;
    println!("Parsed term:\n{parsed}");
    let evaled = eval(parsed, EvalOrder::CBV);
    println!("Evaled term:\n{evaled}");
    Ok(())
}
