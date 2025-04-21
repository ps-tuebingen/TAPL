use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::types::Type;
use common::types::Exists;
use pest::iterators::Pair;

pub fn pair_to_exists(p: Pair<'_, Rule>) -> Result<Exists<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Kind", "Exists Body"])?;
    let var = pair_to_n_inner(inner.remove(0), vec!["Exists Variable"])?
        .remove(0)
        .as_str()
        .trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Exists::new(var, kind, body))
}
