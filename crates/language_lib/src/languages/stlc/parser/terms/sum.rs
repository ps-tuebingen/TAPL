use super::{get_n_inner, pair_to_primterm, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{Left, Right};

pub fn pair_to_left(p: Pair<'_, Rule>) -> Result<Left<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["Inl Argument", "Inl Type"])?;

    let arg_pair = inner.remove(0);
    let arg_term = pair_to_primterm(arg_pair)?;

    let ty_pair = inner.remove(0);
    let ty = pair_to_type(ty_pair)?;

    Ok(Left::new(arg_term, ty))
}

pub fn pair_to_right(p: Pair<'_, Rule>) -> Result<Right<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["Inr Argument", "Inr Type"])?;

    let arg_pair = inner.remove(0);
    let arg_term = pair_to_primterm(arg_pair)?;

    let ty_pair = inner.remove(0);
    let ty = pair_to_type(ty_pair)?;

    Ok(Right::new(arg_term, ty))
}
