use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule, UnknownKeyword},
    pair_to_n_inner, LangParser, Parse, Rule,
};
use pest::{iterators::Pair, Parser};
mod terms;
mod types;
use terms::pair_to_term;
use types::pair_to_type;

impl Parse for Term {
    type ParseError = Error;

    fn rule() -> Rule {
        Rule::term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError> {
        pair_to_term(p)
    }
}

impl Parse for Type {
    type ParseError = Error;
    fn rule() -> Rule {
        Rule::r#type
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError> {
        pair_to_type(p)
    }
}

pub fn parse(input: String) -> Result<Term, Error> {
    let mut parsed = LangParser::parse(Rule::program, &input)?;
    let prog_rule = parsed
        .next()
        .ok_or(<MissingInput as Into<ParserError>>::into(
            MissingInput::new("Program"),
        ))?;

    if let Some(n) = parsed.next() {
        return Err(
            <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!("{n:?}")))
                .into(),
        );
    }

    let prog_inner = pair_to_n_inner(prog_rule, vec!["Term", "EOI"])?.remove(0);
    let term = pair_to_term(prog_inner)?;
    Ok(term)
}
