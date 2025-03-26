use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule};
use crate::syntax::{Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_term = pair_to_prim_term(arg_rule)?;
    Ok(Succ {
        term: Box::new(arg_term),
    })
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(Pred {
        term: Box::new(arg),
    })
}
