use crate::syntax::ClassTable;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/featherweight.pest"]
struct FeatherweightParser;

pub fn parse(input: String) -> Result<ClassTable, Error> {
    let mut parsed = FeatherweightParser::parse(Rule::program, &input)?;
    println!("parsed: {parsed:?}");
    todo!()
}
