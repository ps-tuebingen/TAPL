use crate::syntax::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/exceptions.pest"]
struct ExceptionsParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExceptionsParser::parse(Rule::program, &input)?;
    todo!()
}
