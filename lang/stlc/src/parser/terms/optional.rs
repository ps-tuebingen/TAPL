use super::pair_to_term;
use crate::{
    parser::{get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::{Nothing, Something},
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_some(p: Pair<'_, Rule>) -> Result<Something, Error> {
    let arg_pair = get_n_inner(p, vec!["Something Argument"])?.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::paren_term)?;
    let arg = pair_to_term(arg_rule)?;
    Ok(Something {
        term: Box::new(arg),
    })
}
pub fn pair_to_none(p: Pair<'_, Rule>) -> Result<Nothing, Error> {
    let ty_pair = get_n_inner(p, vec!["Nothing Type"])?.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;
    Ok(Nothing { inner_type: ty })
}
