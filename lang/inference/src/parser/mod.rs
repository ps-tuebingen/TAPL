use crate::syntax::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
mod terms;
mod types;
use errors::Error;
use terms::pair_to_term;
use types::pair_to_type;

#[derive(Parser)]
#[grammar = "parser/inference.pest"]
struct InferenceParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = InferenceParser::parse(Rule::program, &input)?;
    let prog_rule = parsed
        .next()
        .ok_or(Error::MissingInput("Program".to_owned()))?;
    if let Some(n) = parsed.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }

    let mut prog_inner = prog_rule.into_inner();
    let term_rule = prog_inner
        .next()
        .ok_or(Error::MissingInput("Program Term".to_owned()))?;
    prog_inner
        .next()
        .ok_or(Error::MissingInput("EOI".to_owned()))?;
    if let Some(n) = prog_inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
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
        let next = inner.next().ok_or(Error::MissingInput(name.to_owned()))?;
        pairs.push(next);
    }

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(pairs)
}
