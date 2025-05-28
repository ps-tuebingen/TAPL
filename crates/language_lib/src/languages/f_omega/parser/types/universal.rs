use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule, Type};
use pest::iterators::Pair;
use syntax::{kinds::Kind, types::Forall};

pub fn pair_to_universal(p: Pair<'_, Rule>) -> Result<Forall<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Kind", "Forall Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
        .remove(0)
        .as_str()
        .trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Forall::new(var, kind, body))
}

pub fn pair_to_universal_star(p: Pair<'_, Rule>) -> Result<Forall<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
        .remove(0)
        .as_str()
        .trim();
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Forall::new(var, Kind::Star, body))
}
