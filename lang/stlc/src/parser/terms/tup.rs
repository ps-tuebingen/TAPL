use super::pair_to_term;
use crate::{
    parser::{errors::Error, next_rule, Rule},
    syntax::Tup,
};
use pest::iterators::Pair;

pub fn pair_to_tup(p: Pair<'_, Rule>) -> Result<Tup, Error> {
    let mut terms = vec![];
    for p in p.into_inner() {
        let p_rule = next_rule(p, Rule::term)?;
        let p_term = pair_to_term(p_rule)?;
        terms.push(p_term);
    }
    Ok(Tup { terms })
}
