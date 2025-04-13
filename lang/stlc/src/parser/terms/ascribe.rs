use crate::{
    parser::{get_n_inner, types::pair_to_type, Rule},
    syntax::{Ascribe, Term},
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_ascribe(p: Pair<'_, Rule>, t: Term) -> Result<Ascribe, Error> {
    let mut inner = get_n_inner(p, vec!["Ascribed Type"])?;

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(Ascribe {
        term: Box::new(t),
        ty,
    })
}
