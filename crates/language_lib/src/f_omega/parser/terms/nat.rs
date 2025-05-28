use super::{pair_to_n_inner, pair_to_primterm, Error, Rule, Term};
use common::terms::{IsZero, Pred, Succ};
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

pub fn pair_to_iszero(p: Pair<'_, Rule>) -> Result<IsZero<Term>, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["IsZero Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(IsZero::new(arg))
}
