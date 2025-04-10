use super::pair_to_term;
use crate::{
    parser::{pair_to_n_inner, Rule},
    syntax::If,
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "If Keyword",
            "If Condition",
            "Then Term",
            "Else Keyword",
            "Else Term",
        ],
    )?;

    inner.remove(0);
    let ift_rule = inner.remove(0);
    let ift = pair_to_term(ift_rule)?;
    let thent_rule = inner.remove(0);
    let thent = pair_to_term(thent_rule)?;
    inner.remove(0);
    let elset_rule = inner.remove(0);
    let elset = pair_to_term(elset_rule)?;
    Ok(If {
        ift: Box::new(ift),
        thent: Box::new(thent),
        elset: Box::new(elset),
    })
}
