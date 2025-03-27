use crate::syntax::Term;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/lambda_omega.pest"]
struct LambdaOmegaParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LambdaOmegaParser::parse(Rule::program, &input)?;
    todo!()
}
