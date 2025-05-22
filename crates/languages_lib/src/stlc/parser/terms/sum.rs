use super::{get_n_inner, next_rule, pair_to_term, pair_to_type, Rule, Term};
use common::{
    errors::Error,
    terms::{Left, Right},
};
use pest::iterators::Pair;

pub fn pair_to_left(p: Pair<'_, Rule>) -> Result<Left<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["Inl Argument", "Inl Type"])?;

    let arg_pair = inner.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::paren_term)?;
    let arg_term = pair_to_term(arg_rule)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Left::new(arg_term, ty))
}

pub fn pair_to_right(p: Pair<'_, Rule>) -> Result<Right<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["Inr Argument", "Inr Type"])?;

    let arg_pair = inner.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::paren_term)?;
    let arg_term = pair_to_term(arg_rule)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Right::new(arg_term, ty))
}
