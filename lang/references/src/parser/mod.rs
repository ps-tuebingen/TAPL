use crate::terms::Term;
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod terms;
mod types;
use terms::pair_to_term;

pub fn to_parse_err<T>(knd: T) -> Error
where
    T: Into<ErrorKind>,
{
    Error {
        kind: knd.into(),
        loc: ErrorLocation::Parse,
    }
}

#[derive(Parser)]
#[grammar = "parser/references.pest"]
struct ReferencesParser;

impl Parse for Term {
    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ReferencesParser::parse(Rule::program, &input).map_err(to_parse_err)?;
    let prog_rule = parsed
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Program".to_owned())))?;
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }

    let mut prog_inner = prog_rule.into_inner();
    let term_rule = prog_inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Term".to_owned())))?;
    prog_inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("EOI".to_owned())))?;
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    pair_to_term(term_rule)
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut p_inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        let p_next = p_inner
            .next()
            .ok_or(to_parse_err(ErrorKind::MissingInput(name.to_owned())))?;
        pairs.push(p_next);
    }
    if let Some(n) = p_inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(pairs)
}
