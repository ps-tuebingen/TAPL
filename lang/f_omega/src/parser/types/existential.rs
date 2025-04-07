use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::types::Existential;
use pest::iterators::Pair;

pub fn pair_to_exists(p: Pair<'_, Rule>) -> Result<Existential, Error> {
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
        ty_var: var,
        kind,
        ty: Box::new(body),
    })
}
