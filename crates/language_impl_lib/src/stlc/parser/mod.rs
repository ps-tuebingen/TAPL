use super::{terms::Term, types::Type};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod terms;
mod types;
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

#[derive(Parser)]
#[grammar = "stlc/parser/stlc.pest"]
struct StlcParser;

impl Parse for Term {
    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = StlcParser::parse(Rule::program, &input).map_err(to_parse_err)?;
    let prog_pair = parsed
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Program".to_owned())))?;
    let rule = prog_pair.as_rule();
    if rule != Rule::program {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{rule:?}"),
            expected: "Program".to_owned(),
        }));
    }
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }

    let mut prog_inner = prog_pair.into_inner();
    let term_pair = prog_inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Term".to_owned())))?;
    if let Some(n) = prog_inner.next() {
        if n.as_rule() != Rule::EOI {
            return Err(to_parse_err(ErrorKind::RemainingInput(format!(
                "{:?}",
                n.as_rule()
            ))));
        }
    }
    let term = pair_to_term(term_pair)?;
    Ok(term)
}

fn next_rule(p: Pair<'_, Rule>, r: Rule) -> Result<Pair<'_, Rule>, Error> {
    let rule = p.as_rule();
    if rule != r {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{rule:?}"),
            expected: format!("{r:?}"),
        }));
    }
    let mut inner = p.into_inner();
    let next_rule = inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput(format!("{r:?}"))))?;
    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(next_rule)
}

pub fn get_n_inner<'a>(p: Pair<'a, Rule>, names: Vec<&str>) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        pairs.push(
            inner
                .next()
                .ok_or(to_parse_err(ErrorKind::MissingInput(name.to_owned())))?,
        );
    }
    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(pairs)
}
