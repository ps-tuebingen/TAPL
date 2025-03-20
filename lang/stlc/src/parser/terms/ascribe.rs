use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::Ascribe,
};
use pest::iterators::Pair;

pub fn pair_to_ascribe(p: Pair<'_, Rule>) -> Result<Ascribe, Error> {
    let mut inner = get_n_inner(p, vec!["Ascribed Term", "Ascribed Type"])?;
    let term_pair = inner.remove(0);
    let term_rule = next_rule(term_pair, Rule::term)?;
    let term = pair_to_term(term_rule)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Ascribe {
        term: Box::new(term),
        ty,
    })
}
