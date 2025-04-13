use super::pair_to_term;
use crate::{
    parser::{get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::Lambda,
};
use common::errors::Error;
use pest::iterators::Pair;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda, Error> {
    let mut inner = get_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Bodhy"])?;

    let var = inner.remove(0).as_str().trim();

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    let paren_term_rule = inner.remove(0);
    let term_rule = next_rule(paren_term_rule, Rule::paren_term)?;
    let term = pair_to_term(term_rule)?;

    Ok(Lambda {
        var: var.to_owned(),
        annot: ty,
        body: Box::new(term),
    })
}
