use crate::{
    parser::{get_n_inner, types::pair_to_type, Rule},
    terms::Term,
};
use common::{errors::Error, terms::Ascribe};
use pest::iterators::Pair;

pub fn pair_to_ascribe(p: Pair<'_, Rule>, t: Term) -> Result<Ascribe<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["Ascribed Type"])?;

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(Ascribe::new(t, ty))
}
