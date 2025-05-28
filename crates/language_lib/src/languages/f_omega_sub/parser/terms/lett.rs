use super::{pair_to_n_inner, pair_to_term, Error, Rule, Term};
use common::terms::Let;
use pest::iterators::Pair;

pub fn pair_to_let(p: Pair<'_, Rule>) -> Result<Let<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Let Variable", "Let Bound Term", "Let In Term"])?;
    let var = inner.remove(0).as_str().trim();
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Let::new(var, bound_term, in_term))
}
