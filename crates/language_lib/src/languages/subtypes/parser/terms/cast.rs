use super::{pair_to_n_inner, pair_to_type, Rule, Term};
use common::{errors::Error, terms::Cast};
use pest::iterators::Pair;

pub fn pair_to_cast(p: Pair<'_, Rule>, t: Term) -> Result<Cast<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["As Keyword", "Cast Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_pair)?;
    Ok(Cast::new(t, ty))
}
