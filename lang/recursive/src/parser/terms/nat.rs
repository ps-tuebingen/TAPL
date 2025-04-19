use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule};
use crate::terms::Term;
use common::terms::{IsZero, Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(Succ::new(arg))
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(Pred::new(arg))
}

pub fn pair_to_iszero(p: Pair<'_, Rule>) -> Result<IsZero<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["IsZero Keyword", "IsZero Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(IsZero::new(arg))
}
