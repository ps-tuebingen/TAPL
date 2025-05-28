use super::{pair_to_n_inner, pair_to_term, pair_to_type, Rule, Term};
use common::errors::Error;
use pest::iterators::Pair;
use syntax::terms::Lambda;

pub fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "LambdaBody"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let ty_pair = inner.remove(0);
    let ty_rule = pair_to_n_inner(ty_pair, vec!["Type"])?.remove(0);
    let annot = pair_to_type(ty_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;

    Ok(Lambda {
        var,
        annot,
        body: Box::new(term),
    })
}
