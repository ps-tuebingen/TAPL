use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::{kinds::Kind, types::Universal};
use pest::iterators::Pair;

pub fn pair_to_universal(p: Pair<'_, Rule>) -> Result<Universal, Error> {
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
        kind,
        ty: Box::new(body),
    })
}

pub fn pair_to_universal_star(p: Pair<'_, Rule>) -> Result<Universal, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
        .remove(0)
        .as_str()
        .trim()
        .to_owned();
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Universal {
        var,
        kind: Kind::Star,
        ty: Box::new(body),
    })
}
