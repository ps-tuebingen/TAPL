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
use types::pair_to_type;

#[derive(Parser)]
#[grammar = "parser/bounded.pest"]
struct BoundedParser;

impl Parse for Term {
    type Err = Error;
    fn parse(input: String) -> Result<Self, Self::Err> {
        parse(input)
    }
}

pub fn to_parse_err<T>(knd: T) -> Error
where
    T: Into<ErrorKind>,
{
    to_err(knd.into(), ErrorLocation::Parse)
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = BoundedParser::parse(Rule::program, &input).map_err(to_parse_err)?;
    let prog_rule = parsed
        .next()
        .ok_or(ErrorKind::MissingInput("Program".to_owned()))
        .map_err(to_parse_err)?;

    if let Some(n) = parsed.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
    }

    let prog_inner = pair_to_n_inner(prog_rule, vec!["Term", "EOI"])?.remove(0);
    let term = pair_to_term(prog_inner)?;
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
            .ok_or(ErrorKind::MissingInput(name.to_owned()))
            .map_err(to_parse_err)?;
        pairs.push(next)
    }
    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
    }
    Ok(pairs)
}
