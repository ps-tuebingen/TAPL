use crate::syntax::terms::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
pub mod kinds;
pub mod terms;
pub mod types;
use errors::Error;
use kinds::pair_to_kind;
use terms::pair_to_term;
use types::pair_to_type;

#[derive(Parser)]
#[grammar = "parser/fomega.pest"]
struct FOmegaParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = FOmegaParser::parse(Rule::program, &input)?
        .next()
        .ok_or(Error::missing("Program"))?
        .into_inner();
    let term_rule = parsed.next().ok_or(Error::missing("Term"))?;
    let term = pair_to_term(term_rule)?;
    parsed.next().ok_or(Error::missing("EOI"))?;
    if let Some(n) = parsed.next() {
        return Err(Error::remaining(&n));
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
        let next = inner.next().ok_or(Error::missing(name))?;
        pairs.push(next);
    }
    if let Some(n) = inner.next() {
        return Err(Error::remaining(&n));
    }

    Ok(pairs)
}
