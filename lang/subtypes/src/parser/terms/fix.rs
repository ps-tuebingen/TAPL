use super::pair_to_prim_term;
use crate::{
    parser::{pair_to_n_inner, Rule},
    syntax::Fix,
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_fix(p: Pair<'_, Rule>) -> Result<Fix, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fix Keyword", "Fix Term"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Fix {
        term: Box::new(term),
    })
}
