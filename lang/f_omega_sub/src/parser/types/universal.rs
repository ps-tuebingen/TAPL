use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::types::{Type, Universal};
use pest::iterators::Pair;

pub fn pair_to_universal(p: Pair<'_, Rule>) -> Result<Universal, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Forall Variable", "Forall Super Type", "Forall Body"],
    )?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    let sup_ty_rule = inner.remove(0);
    let sup_ty = pair_to_type(sup_ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Universal {
        var,
        sup_ty: Box::new(sup_ty),
        ty: Box::new(body),
    })
}

pub fn pair_to_universal_unbounded(p: Pair<'_, Rule>) -> Result<Universal, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Kind", "Forall Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Universal {
        var,
        sup_ty: Box::new(Type::Top(kind)),
        ty: Box::new(body),
    })
}
