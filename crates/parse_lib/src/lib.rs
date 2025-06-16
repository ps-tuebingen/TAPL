pub mod errors;
pub mod kinds;
pub mod terms;
pub mod types;

use errors::{MissingInput, ParserError, RemainingInput};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../parse_lib/src/grammar.pest"]
pub struct LangParser;

pub trait Parse: Sized {
    type ParseError: std::error::Error + From<pest::error::Error<Rule>> + From<ParserError>;
    type LeftRecArg;

    fn rule() -> Rule;

    fn from_pair(p: Pair<'_, Rule>, left_rec: Self::LeftRecArg) -> Result<Self, Self::ParseError>;

    fn parse(source: String) -> Result<Self, Self::ParseError>
    where
        Self::LeftRecArg: Default,
    {
        let mut pairs = LangParser::parse(Self::rule(), &source)?;
        let rule = pairs
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new(&format!("{:?}", Self::rule())),
            ))?;
        let result = Self::from_pair(rule, Default::default())?;
        if let Some(rule) = pairs.next() {
            Err(
                <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!(
                    "{:?}",
                    rule
                )))
                .into(),
            )
        } else {
            Ok(result)
        }
    }
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, ParserError> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let next = inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(next)
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }
    Ok(pairs)
}
