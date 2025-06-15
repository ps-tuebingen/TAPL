use super::{get_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::Lambda;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Bodhy"])?;

    let var = inner.remove(0).as_str().trim();

    let ty_pair = inner.remove(0);
    let ty = pair_to_type(ty_pair)?;

    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;

    Ok(Lambda::new(var, ty, term))
}
