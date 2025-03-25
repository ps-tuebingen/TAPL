use crate::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/inference.pest"]
struct InferenceParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = InferenceParser::parse(Rule::program, &input)?;
    todo!()
}
