use crate::terms::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
pub mod terms;
pub mod types;
use errors::Error;
use terms::pair_to_term;
use types::pair_to_type;

#[derive(Parser)]
#[grammar = "parser/existential.pest"]
struct ExistentialParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ExistentialParser::parse(Rule::program, &input)?
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
        let pair = inner.next().ok_or(Error::missing(name))?;
        pairs.push(pair);
    }
    if let Some(n) = inner.next() {
        return Err(Error::remaining(&n));
    }
    Ok(pairs)
}
