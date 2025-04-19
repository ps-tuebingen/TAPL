use super::{pair_to_n_inner, pair_to_term, Error, Rule};
use crate::syntax::Lambda;
use pest::iterators::Pair;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Lambda {
        var,
        body: Box::new(term),
    })
}
