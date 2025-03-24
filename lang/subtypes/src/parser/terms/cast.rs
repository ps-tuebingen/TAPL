use crate::{
    parser::{errors::Error, pair_to_n_inner, types::pair_to_type, Rule},
    syntax::{Cast, Term},
};
use pest::iterators::Pair;

pub fn pair_to_cast(p: Pair<'_, Rule>, t: Term) -> Result<Cast, Error> {
    let mut inner = pair_to_n_inner(p, vec!["As Keyword", "Cast Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Cast {
        term: Box::new(t),
        ty,
    })
}
