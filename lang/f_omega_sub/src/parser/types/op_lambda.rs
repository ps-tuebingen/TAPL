use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::types::Type;
use common::types::{OpLambdaSub, Top};
use pest::iterators::Pair;

pub fn pair_to_op_lambda(p: Pair<'_, Rule>) -> Result<OpLambdaSub<Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(OpLambdaSub::new(var, ty, body))
}

pub fn pair_to_op_lambda_star(p: Pair<'_, Rule>) -> Result<OpLambdaSub<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Op Lambda Var", "Op Lambda Kind", "Op Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(OpLambdaSub::new(var, Top::new(kind), body))
}
