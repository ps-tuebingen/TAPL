use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
    Parse,
};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

mod terms;
mod types;
use terms::pair_to_term;
use types::pair_to_type;

#[derive(Parser)]
#[grammar = "languages/stlc/parser/stlc.pest"]
struct StlcParser;

impl Parse for Term {
    type Rule = Rule;
    type ParseError = Error;

    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = StlcParser::parse(Rule::program, &input)?;
    let prog_pair = parsed.next().ok_or(MissingInput::new("Program"))?;
    let rule = prog_pair.as_rule();
    if rule != Rule::program {
        return Err(UnexpectedRule::new(rule, "Program").into());
    }
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }

    let mut prog_inner = prog_pair.into_inner();
    let term_pair = prog_inner.next().ok_or(MissingInput::new("Term"))?;
    if let Some(n) = prog_inner.next() {
        if n.as_rule() != Rule::EOI {
            return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
        }
    }
    let term = pair_to_term(term_pair)?;
    Ok(term)
}

fn next_rule(p: Pair<'_, Rule>, r: Rule) -> Result<Pair<'_, Rule>, Error> {
    let rule = p.as_rule();
    if rule != r {
        return Err(UnexpectedRule::new(rule, &format!("{r:?}")).into());
    }
    let mut inner = p.into_inner();
    let next_rule = inner.next().ok_or(MissingInput::new(&format!("{r:?}")))?;
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(next_rule)
}

pub fn get_n_inner<'a>(p: Pair<'a, Rule>, names: Vec<&str>) -> Result<Vec<Pair<'a, Rule>>, Error> {
    let mut inner = p.into_inner();
    let mut pairs = vec![];
    for name in names {
        pairs.push(inner.next().ok_or(MissingInput::new(name))?);
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(pairs)
}
