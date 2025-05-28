use super::{get_n_inner, pair_to_term, Rule, Term};
use common::errors::Error;
use pest::iterators::Pair;
use syntax::terms::Let;

pub fn pair_to_let(p: Pair<'_, Rule>) -> Result<Let<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["Let Variable", "Let Bound Term", "Let In Term"])?;

    let var_pair = inner.remove(0);
    let var = var_pair.as_str().trim().to_owned();

    let bound_pair = inner.remove(0);
    let bound_term = pair_to_term(bound_pair)?;

    let in_pair = inner.remove(0);
    let in_term = pair_to_term(in_pair)?;

    Ok(Let::new(&var, bound_term, in_term))
}
