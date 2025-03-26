use crate::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/bounded.pest"]
struct BoundedParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = BoundedParser::parse(Rule::program, &input)?;
    todo!()
}
