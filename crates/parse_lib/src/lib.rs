pub mod errors;
pub mod terms;

use errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../parse_lib/src/grammar.pest"]
pub struct LangParser;

pub trait Parse: Sized {
    type ParseError: std::error::Error + From<pest::error::Error<Rule>> + From<ParserError>;

    fn rule() -> Rule;

    fn from_pair(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError>;

    fn parse(source: String) -> Result<Self, Self::ParseError> {
        let mut pairs = LangParser::parse(Self::rule(), &source)?;
        let rule = pairs
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new(&format!("{:?}", Self::rule())),
            ))?;
        let result = Self::from_pair(rule)?;
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

pub fn to_paren_term_inner<'a>(p: Pair<'a, Rule>) -> Result<Pair<'a, Rule>, ParserError> {
    if p.as_rule() != Rule::paren_term {
        return Err(UnexpectedRule::new(p.as_rule(), "Paren Term").into());
    }
    let mut inner = p.into_inner();
    let rule = inner.next().ok_or(MissingInput::new("Paren Term inner"))?;

    if let Some(r) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", r)).into());
    }

    Ok(rule)
}
