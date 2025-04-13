use super::pair_to_term;
use crate::{parser::Rule, syntax::Tup};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_tup(p: Pair<'_, Rule>) -> Result<Tup, Error> {
    let mut terms = vec![];
    for p in p.into_inner() {
        let p_term = pair_to_term(p)?;
        terms.push(p_term);
    }
    Ok(Tup { terms })
}
