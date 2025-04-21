use super::{pair_to_n_inner, pair_to_primterm, Error, Rule};
use crate::terms::Term;
use common::terms::{Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ<Term>, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["Succ Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(Succ::new(arg))
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred<Term>, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["Pred Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(Pred::new(arg))
}
