use crate::syntax::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod errors;
use errors::Error;

mod terms;
mod types;
use terms::pair_to_term;

#[derive(Parser)]
#[grammar = "parser/stlc.pest"]
struct StlcParser;

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = StlcParser::parse(Rule::program, &input)?;
    println!("trying to parse program");
    let prog_pair = parsed
        .next()
        .ok_or(Error::MissingInput("Program".to_owned()))?;
    let rule = prog_pair.as_rule();
    if rule != Rule::program {
        return Err(Error::UnexpectedRule {
            found: rule,
            expected: "Program".to_owned(),
        });
    }
    if let Some(n) = parsed.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }

    let mut prog_inner = prog_pair.into_inner();
    let term_pair = prog_inner
        .next()
        .ok_or(Error::MissingInput("Term".to_owned()))?;
    if let Some(n) = prog_inner.next() {
        if n.as_rule() != Rule::EOI {
            return Err(Error::RemainingInput(n.as_rule()));
        }
    }
    let term = pair_to_term(term_pair)?;
    println!("got term {term}");
    Ok(term)
}

fn next_rule(p: Pair<'_, Rule>, r: Rule) -> Result<Pair<'_, Rule>, Error> {
    let rule = p.as_rule();
    if rule != r {
        return Err(Error::UnexpectedRule {
            found: rule,
            expected: format!("{r:?}"),
        });
    }
    let mut inner = p.into_inner();
    let next_rule = inner.next().ok_or(Error::EmptyInput)?;
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(next_rule)
}

pub fn get_n_inner<'a>(p: Pair<'a, Rule>, names: Vec<&str>) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        pairs.push(inner.next().ok_or(Error::MissingInput(name.to_owned()))?);
    }
    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(pairs)
}
