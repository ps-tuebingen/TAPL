use super::{pair_to_n_inner, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::Cast;

pub fn pair_to_cast(p: Pair<'_, Rule>, t: Term) -> Result<Cast<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Cast Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Cast::new(t, ty))
}
