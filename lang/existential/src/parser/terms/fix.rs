use super::{pair_to_n_inner, pair_to_primterm, Error, Rule};
use crate::terms::Term;
use common::terms::Fix;
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fix Keyword", "Fix Term"])?;
    inner.remove(0);
    let inner_rule = inner.remove(0);
    let inner = pair_to_primterm(inner_rule)?;
    Ok(Fix::new(inner))
}
