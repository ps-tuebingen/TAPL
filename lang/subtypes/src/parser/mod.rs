use crate::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/subtypes.pest"]
struct SubtypesParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = SubtypesParser::parse(Rule::program, &input)?;
    todo!()
}
