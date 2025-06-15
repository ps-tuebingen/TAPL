use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod terms;
pub mod types;
use terms::pair_to_term;
use types::pair_to_type;

impl Parse for Term {
    type Rule = Rule;
    type ParseError = Error;

    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

#[derive(Parser)]
#[grammar = "languages/existential/parser/existential.pest"]
struct ExistentialParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExistentialParser::parse(Rule::program, &input)?
        .next()
        .ok_or(MissingInput::new("Program"))?
        .into_inner();
    let term_rule = parsed.next().ok_or(MissingInput::new("Term"))?;
    let term = pair_to_term(term_rule)?;

    parsed.next().ok_or(MissingInput::new("EOI"))?;
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n)).into());
    }
    Ok(term)
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let pair = inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(pair);
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n)).into());
    }
    Ok(pairs)
}
