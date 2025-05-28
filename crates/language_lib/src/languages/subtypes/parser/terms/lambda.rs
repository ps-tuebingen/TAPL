use super::{pair_to_n_inner, pair_to_term, pair_to_type, Rule, Term};
use common::errors::Error;
use pest::iterators::Pair;
use syntax::terms::Lambda;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();

    let ty_rule = inner.remove(0);
    let ty_inner = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let annot = pair_to_type(ty_inner)?;

    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;

    Ok(Lambda::new(var, annot, term))
}
