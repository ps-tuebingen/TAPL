use crate::terms::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/references.pest"]
struct ReferencesParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ReferencesParser::parse(Rule::program, &input)?;
    todo!()
}
