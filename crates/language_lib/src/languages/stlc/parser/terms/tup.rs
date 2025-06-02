use super::{pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::Tuple;

pub fn pair_to_tup(p: Pair<'_, Rule>) -> Result<Tuple<Term>, Error> {
    let mut terms = vec![];
    for p in p.into_inner() {
        let p_term = pair_to_term(p)?;
        terms.push(p_term);
    }
    Ok(Tuple::new(terms))
}
