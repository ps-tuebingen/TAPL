use crate::terms::Term;
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod kinds;
mod terms;
mod types;
use kinds::pair_to_kind;
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
#[grammar = "parser/lambda_omega.pest"]
struct LambdaOmegaParser;

impl Parse for Term {
    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LambdaOmegaParser::parse(Rule::program, &input).map_err(to_parse_err)?;
    let prog_rule = parsed
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Program".to_owned())))?;
    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }

    let term_rule = pair_to_n_inner(prog_rule, vec!["Term", "EOI"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(term)
}

pub fn pair_to_n_inner<'a>(
    p: Pair<'a, Rule>,
    names: Vec<&str>,
) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut pairs = vec![];
    let mut inner = p.into_inner();
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
