use crate::terms::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
mod terms;
mod types;
use errors::Error;
use terms::pair_to_term;

#[derive(Parser)]
#[grammar = "parser/references.pest"]
struct ReferencesParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = ReferencesParser::parse(Rule::program, &input)?;
    let prog_rule = parsed.next().ok_or(Error::missing("Program"))?;
    if let Some(n) = parsed.next() {
        return Err(Error::remaining(n.as_rule()));
    }

    let mut prog_inner = prog_rule.into_inner();
    let term_rule = prog_inner.next().ok_or(Error::missing("Term"))?;
    prog_inner.next().ok_or(Error::missing("EOI"))?;
    if let Some(n) = parsed.next() {
        return Err(Error::remaining(n.as_rule()));
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
        let p_next = p_inner.next().ok_or(Error::missing(name))?;
        pairs.push(p_next);
    }
    if let Some(n) = p_inner.next() {
        return Err(Error::remaining(n.as_rule()));
    }
    Ok(pairs)
}
