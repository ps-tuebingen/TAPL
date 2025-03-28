use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule};
use crate::terms::{IsZero, Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Succ Argument"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Succ {
        term: Box::new(arg),
    })
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Pred Argument"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Pred {
        term: Box::new(arg),
    })
}

pub fn pair_to_iszero(p: Pair<'_, Rule>) -> Result<IsZero, Error> {
    let mut inner = pair_to_n_inner(p, vec!["IsZero Keyword", "IsZero Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["IsZero Argument"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(IsZero {
        term: Box::new(arg),
    })
}
