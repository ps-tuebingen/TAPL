use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::types::{Existential, Type};
use pest::iterators::Pair;

pub fn pair_to_exists(p: Pair<'_, Rule>) -> Result<Existential, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Exists Variable", "Exists Super Type", "Exists Body"],
    )?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Exists Variable"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    let sup_ty_rule = inner.remove(0);
    let sup_ty = pair_to_type(sup_ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Existential {
        var,
        sup_ty: Box::new(sup_ty),
        ty: Box::new(body),
    })
}

pub fn pair_to_exists_unbounded(p: Pair<'_, Rule>) -> Result<Existential, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Kind", "Exists Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Exists Variable"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Existential {
        var,
        sup_ty: Box::new(Type::Top(kind)),
        ty: Box::new(body),
    })
}
