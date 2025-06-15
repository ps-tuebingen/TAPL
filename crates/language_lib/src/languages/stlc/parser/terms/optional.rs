use super::{get_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{Nothing, Something};

pub fn pair_to_some(p: Pair<'_, Rule>) -> Result<Something<Term>, Error> {
    let arg_pair = get_n_inner(p, vec!["Something Argument"])?.remove(0);
    let arg = pair_to_term(arg_pair)?;
    Ok(Something::new(arg))
}
pub fn pair_to_none(p: Pair<'_, Rule>) -> Result<Nothing<Term, Type>, Error> {
    let ty_pair = get_n_inner(p, vec!["Nothing Type"])?.remove(0);
    let ty = pair_to_type(ty_pair)?;
    Ok(Nothing::new(ty))
}
