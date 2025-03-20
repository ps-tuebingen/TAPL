use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::{App, Lambda},
};
use pest::iterators::Pair;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda, Error> {
    let mut inner = get_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Bodhy"])?;

    let var = inner.remove(0).as_str().trim();

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    let term_pair = inner.remove(0);
    let term_rule = next_rule(term_pair, Rule::term)?;
    let term = pair_to_term(term_rule)?;

    Ok(Lambda {
        var: var.to_owned(),
        annot: ty,
        body: Box::new(term),
    })
}

pub fn pair_to_app(p: Pair<'_, Rule>) -> Result<App, Error> {
    let mut inner = get_n_inner(p, vec!["Function Term", "Argument"])?;

    let fun_pair = inner.remove(0);
    let fun_rule = next_rule(fun_pair, Rule::term)?;
    let fun_term = pair_to_term(fun_rule)?;

    let arg_pair = inner.remove(0);
    let arg_rule = next_rule(arg_pair, Rule::term)?;
    let arg_term = pair_to_term(arg_rule)?;

    Ok(App {
        fun: Box::new(fun_term),
        arg: Box::new(arg_term),
    })
}
