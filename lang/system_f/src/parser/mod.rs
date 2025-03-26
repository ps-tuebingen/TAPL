use crate::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/systemf.pest"]
struct SystemFParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = SystemFParser::parse(Rule::program, &input)?;
    todo!()
}
