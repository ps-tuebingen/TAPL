use super::{pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::{Ascribe, Term};
use pest::iterators::Pair;

pub fn pair_to_ascribe(p: Pair<'_, Rule>, t: Term) -> Result<Ascribe, Error> {
    let mut inner = pair_to_n_inner(p, vec!["As Keyword", "Ascription Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Ascribe {
        term: Box::new(t),
        ty,
    })
}
