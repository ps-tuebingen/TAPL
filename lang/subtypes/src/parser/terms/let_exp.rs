use super::pair_to_term;
use crate::{
    parser::{pair_to_n_inner, Rule},
    terms::Term,
};
use common::{errors::Error, terms::Let};
use pest::iterators::Pair;

pub fn pair_to_let(p: Pair<'_, Rule>) -> Result<Let<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Let Keyword",
            "Let Variable",
            "Let Bound Term",
            "In Keyword",
            "Let In Term",
        ],
    )?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim();
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    inner.remove(0);
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Let::new(var, bound_term, in_term))
}
