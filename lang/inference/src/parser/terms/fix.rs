use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule};
use crate::syntax::Fix;
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fix Keywword", "Fix Term"])?;
    inner.remove(0);
    let prim_rule = inner.remove(0);
    let term = pair_to_prim_term(prim_rule)?;
    Ok(Fix {
        term: Box::new(term),
    })
}
