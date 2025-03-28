use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule};
use crate::terms::Fix;
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Fix Term"])?.remove(0);
    let term_inner = pair_to_n_inner(term_rule, vec!["Fix Term"])?.remove(0);
    let term = pair_to_prim_term(term_inner)?;
    Ok(Fix {
        term: Box::new(term),
    })
}
