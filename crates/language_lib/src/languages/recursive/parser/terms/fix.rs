use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::Fix;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fix Keyword", "Fix Term"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Fix::new(term))
}
