use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, next_rule, Rule},
    syntax::{IsZero, Pred, Succ, Term},
};
use pest::iterators::Pair;

pub fn pair_to_num(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let num_str = p.as_str().trim();
    let num = num_str
        .parse::<i64>()
        .map_err(|_| Error::BadTerm(num_str.to_owned()))?;
    Ok(num.into())
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let inner_pair = get_n_inner(p, vec!["Pred Argument"])?.remove(0);
    let inner_rule = next_rule(inner_pair, Rule::paren_term)?;
    let inner_term = pair_to_term(inner_rule)?;
    Ok(Pred {
        term: Box::new(inner_term),
    })
}

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let inner_pair = get_n_inner(p, vec!["Succ Argument"])?.remove(0);
    let inner_rule = next_rule(inner_pair, Rule::paren_term)?;
    let inner_term = pair_to_term(inner_rule)?;
    Ok(Succ {
        term: Box::new(inner_term),
    })
}

pub fn pair_to_isz(p: Pair<'_, Rule>) -> Result<IsZero, Error> {
    let term_pair = get_n_inner(p, vec!["IsZero Argument"])?.remove(0);
    let term_rule = next_rule(term_pair, Rule::paren_term)?;
    let term = pair_to_term(term_rule)?;
    Ok(IsZero {
        term: Box::new(term),
    })
}
