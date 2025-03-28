use crate::terms::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/recursive.pest"]
struct RecursiveParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = RecursiveParser::parse(Rule::program, &input)?;
    todo!()
}
