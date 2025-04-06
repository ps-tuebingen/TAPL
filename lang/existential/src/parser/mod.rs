use crate::terms::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/existential.pest"]
struct ExistentialParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExistentialParser::parse(Rule::program, &input)?;
    todo!()
}
