use crate::syntax::ClassDeclaration;
use pest::Parser;
use pest_derive::Parser;

pub mod errors;
use errors::Error;

#[derive(Parser)]
#[grammar = "parser/featherweight.pest"]
struct FeatherweightParser;

pub fn parse(input: String) -> Result<ClassDeclaration, Error> {
    let mut parsed = FeatherweightParser::parse(Rule::program, &input)?;

    todo!()
}
