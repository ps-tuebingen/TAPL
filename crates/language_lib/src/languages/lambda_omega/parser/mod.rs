use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
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

#[derive(Parser)]
#[grammar = "../../parse_lib/src/grammar.pest"]
struct LambdaOmegaParser;

impl Parse for Term {
    type Rule = Rule;
    type ParseError = Error;

    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LambdaOmegaParser::parse(Rule::program, &input)?;
    let prog_rule = parsed.next().ok_or(MissingInput::new("Program"))?;
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
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
        let next = inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(next);
    }
    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }

    Ok(pairs)
}
