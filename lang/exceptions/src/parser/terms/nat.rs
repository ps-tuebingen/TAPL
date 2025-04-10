use super::pair_to_prim_term;
use crate::{
    parser::{pair_to_n_inner, Rule},
    syntax::{IsZero, Pred, Succ},
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Succ {
        term: Box::new(term),
    })
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Pred {
        term: Box::new(term),
    })
}

pub fn pair_to_isz(p: Pair<'_, Rule>) -> Result<IsZero, Error> {
    let mut inner = pair_to_n_inner(p, vec!["IsZero Keyword", "IsZero Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(IsZero {
        term: Box::new(term),
    })
}
