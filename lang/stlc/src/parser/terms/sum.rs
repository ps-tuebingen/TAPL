use super::pair_to_term;
use crate::{
    parser::{get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::{Left, Right},
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_left(p: Pair<'_, Rule>) -> Result<Left, Error> {
    let mut inner = get_n_inner(p, vec!["Inl Argument", "Inl Type"])?;

    let arg_pair = inner.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::paren_term)?;
    let arg_term = pair_to_term(arg_rule)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Left {
        left_term: Box::new(arg_term),
        ty,
    })
}

pub fn pair_to_right(p: Pair<'_, Rule>) -> Result<Right, Error> {
    let mut inner = get_n_inner(p, vec!["Inr Argument", "Inr Type"])?;

    let arg_pair = inner.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::paren_term)?;
    let arg_term = pair_to_term(arg_rule)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Right {
        right_term: Box::new(arg_term),
        ty,
    })
}
