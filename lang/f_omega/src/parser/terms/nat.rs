use super::{pair_to_n_inner, pair_to_primterm, Error, Rule};
use crate::syntax::terms::{IsZero, Pred, Succ};
use pest::iterators::Pair;

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["Succ Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(Succ {
        term: Box::new(arg),
    })
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["Pred Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(Pred {
        term: Box::new(arg),
    })
}

pub fn pair_to_iszero(p: Pair<'_, Rule>) -> Result<IsZero, Error> {
    let arg_rule = pair_to_n_inner(p, vec!["IsZero Argument"])?.remove(0);
    let arg = pair_to_primterm(arg_rule)?;
    Ok(IsZero {
        term: Box::new(arg),
    })
}
