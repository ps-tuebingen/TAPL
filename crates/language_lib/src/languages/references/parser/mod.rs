use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule, UnknownKeyword},
    LangParser, Parse, Rule,
};
use pest::{iterators::Pair, Parser};

mod terms;
mod types;
use terms::pair_to_term;

impl Parse for Term {
    type ParseError = Error;

    fn parse(input: String) -> Result<Self, Error> {
        parse(input)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LangParser::parse(Rule::program, &input)?;
    let prog_rule = parsed.next().ok_or(MissingInput::new("Program"))?;
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }

    let mut prog_inner = prog_rule.into_inner();
    let term_rule = prog_inner.next().ok_or(MissingInput::new("Term"))?;
    prog_inner.next().ok_or(MissingInput::new("EOI"))?;
    if let Some(n) = parsed.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
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
        let p_next = p_inner.next().ok_or(MissingInput::new(name))?;
        pairs.push(p_next);
    }
    if let Some(n) = p_inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(pairs)
}
