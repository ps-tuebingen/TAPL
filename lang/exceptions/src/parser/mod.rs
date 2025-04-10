use crate::{syntax::Term, to_err};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod terms;
mod types;
use terms::pair_to_term;

#[derive(Parser)]
#[grammar = "parser/exceptions.pest"]
struct ExceptionsParser;

pub fn to_parse_err<T>(knd: T) -> Error
where
    T: Into<ErrorKind>,
{
    to_err(knd.into(), ErrorLocation::Parse)
}

impl Parse for Term {
    type Err = Error;
    fn parse(input: String) -> Result<Self, Self::Err> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExceptionsParser::parse(Rule::program, &input).map_err(to_parse_err)?;
    let mut prog_inner = parsed
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Program".to_owned())))?
        .into_inner();
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }

    let term_rule = prog_inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Term".to_owned())))?;
    let term = pair_to_term(term_rule)?;

    let _ = prog_inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("EOI".to_owned())))?;
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
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
        let next = inner
            .next()
            .ok_or(to_parse_err(ErrorKind::MissingInput(name.to_owned())))?;
        pairs.push(next);
    }

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }

    Ok(pairs)
}
