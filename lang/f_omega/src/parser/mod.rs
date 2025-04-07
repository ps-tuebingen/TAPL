use crate::syntax::terms::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/fomega.pest"]
struct FOmegaParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = FOmegaParser::parse(Rule::program, &input)?
        .next()
        .ok_or(Error::missing("Program"))?
        .into_inner();
    todo!()
}
