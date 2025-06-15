use super::{pair_to_n_inner, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::Fix;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fix Term"])?;
    let inner_rule = inner.remove(0);
    let inner = pair_to_term(inner_rule)?;
    Ok(Fix::new(inner))
}
