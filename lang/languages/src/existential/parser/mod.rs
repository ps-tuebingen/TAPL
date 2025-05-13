use super::{terms::Term, types::Type};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod terms;
pub mod types;
use terms::pair_to_term;
use types::pair_to_type;

pub fn to_parse_err<T>(knd: T) -> Error
where
    T: Into<ErrorKind>,
{
    Error {
        kind: knd.into(),
        loc: ErrorLocation::Parse,
    }
}

impl Parse for Term {
    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

#[derive(Parser)]
#[grammar = "existential/parser/existential.pest"]
struct ExistentialParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExistentialParser::parse(Rule::program, &input)
        .map_err(to_parse_err)?
        .next()
        .ok_or(ErrorKind::MissingInput("Program".to_owned()))
        .map_err(to_parse_err)?
        .into_inner();
    let term_rule = parsed
        .next()
        .ok_or(ErrorKind::MissingInput("Term".to_owned()))
        .map_err(to_parse_err)?;
    let term = pair_to_term(term_rule)?;

    parsed
        .next()
        .ok_or(ErrorKind::MissingInput("EOI".to_string()))
        .map_err(to_parse_err)?;
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{:?}", n))));
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
        let pair = inner
            .next()
            .ok_or(ErrorKind::MissingInput(name.to_owned()))
            .map_err(to_parse_err)?;
        pairs.push(pair);
    }
    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{:?}", n))));
    }
    Ok(pairs)
}
