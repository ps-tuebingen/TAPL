use super::pair_to_term;
use crate::{parser::Rule, terms::Term};
use common::{errors::Error, terms::Tuple};
use pest::iterators::Pair;

pub fn pair_to_tup(p: Pair<'_, Rule>) -> Result<Tuple<Term>, Error> {
    let mut terms = vec![];
    for p in p.into_inner() {
        let p_term = pair_to_term(p)?;
        terms.push(p_term);
    }
    Ok(Tuple::new(terms))
}
