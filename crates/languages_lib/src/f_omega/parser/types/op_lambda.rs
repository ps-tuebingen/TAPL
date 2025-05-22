use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule, Type};
use common::{kinds::Kind, types::OpLambda};
use pest::iterators::Pair;

pub fn pair_to_op_lambda(p: Pair<'_, Rule>) -> Result<OpLambda<Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Op Lambda Var", "Op Lambda Annot", "Op Lambda Body"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(OpLambda::new(var, kind, body))
}

pub fn pair_to_op_lambda_star(p: Pair<'_, Rule>) -> Result<OpLambda<Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Op Lambda Var", "Op Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(OpLambda::new(var, Kind::Star, body))
}
