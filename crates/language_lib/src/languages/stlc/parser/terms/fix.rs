use super::{get_n_inner, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::Fix;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix<Term>, Error> {
    let inner_rule = get_n_inner(p, vec!["Fix Term"])?.remove(0);
    let inner = pair_to_term(inner_rule)?;
    Ok(Fix::new(inner))
}
