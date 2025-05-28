use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule, Type};
use pest::iterators::Pair;
use syntax::types::{ExistsBounded, Top};

pub fn pair_to_exists(p: Pair<'_, Rule>) -> Result<ExistsBounded<Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Exists Variable", "Exists Super Type", "Exists Body"],
    )?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Exists Variable"])?
        .remove(0)
        .as_str()
        .trim();
    let sup_ty_rule = inner.remove(0);
    let sup_ty = pair_to_type(sup_ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(ExistsBounded::new(var, sup_ty, body))
}

pub fn pair_to_exists_unbounded(p: Pair<'_, Rule>) -> Result<ExistsBounded<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Kind", "Exists Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Exists Variable"])?
        .remove(0)
        .as_str()
        .trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(ExistsBounded::new(var, Top::new(kind), body))
}
