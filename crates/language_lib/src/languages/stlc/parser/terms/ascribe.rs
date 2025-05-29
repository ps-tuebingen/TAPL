use super::{get_n_inner, pair_to_type, Rule, Term, Type};
use common::errors::Error;
use pest::iterators::Pair;
use syntax::terms::Ascribe;

pub fn pair_to_ascribe(p: Pair<'_, Rule>, t: Term) -> Result<Ascribe<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["Ascribed Type"])?;

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(Ascribe::new(t, ty))
}
