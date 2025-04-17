use super::pair_to_term;
use crate::{
    parser::{get_n_inner, Rule},
    terms::Term,
};
use common::{errors::Error, terms::Fix};
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix<Term>, Error> {
    let inner_rule = get_n_inner(p, vec!["Fix Term"])?.remove(0);
    let inner = pair_to_term(inner_rule)?;
    Ok(Fix::new(inner))
}
