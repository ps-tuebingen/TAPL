use super::pair_to_term;
use crate::{
    parser::{errors::Error, pair_to_n_inner, Rule},
    syntax::{IsZero, Pred, Succ},
};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let inner = pair_to_n_inner(p, vec!["Succ Argument"])?.remove(0);
    let term = pair_to_term(inner)?;
    Ok(Succ {
        term: Box::new(term),
    })
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let inner = pair_to_n_inner(p, vec!["Pred Argument"])?.remove(0);
    let term = pair_to_term(inner)?;
    Ok(Pred {
        term: Box::new(term),
    })
}

pub fn pair_to_isz(p: Pair<'_, Rule>) -> Result<IsZero, Error> {
    let inner = pair_to_n_inner(p, vec!["IsZero Argumnet"])?.remove(0);
    let term = pair_to_term(inner)?;
    Ok(IsZero {
        term: Box::new(term),
    })
}
