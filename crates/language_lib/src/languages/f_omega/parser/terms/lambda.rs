use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::Lambda;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Lambda::new(var, ty, body))
}
