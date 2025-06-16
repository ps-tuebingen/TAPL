pub mod errors;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../parse_lib/src/grammar.pest"]
pub struct LangParser;

pub trait Parse: Sized {
    type ParseError: std::error::Error + From<pest::error::Error<Rule>>;
    fn parse(sourcte: String) -> Result<Self, Self::ParseError>;
}
