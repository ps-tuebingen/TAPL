use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule, Term};
use common::terms::{Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_term = pair_to_prim_term(arg_rule)?;
    Ok(Succ::new(arg_term))
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg = pair_to_prim_term(arg_rule)?;
    Ok(Pred::new(arg))
}
