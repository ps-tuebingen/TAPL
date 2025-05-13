use super::{pair_to_kind, pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term};
use common::{
    terms::{LambdaSub, TyApp},
    types::Top,
};
use pest::iterators::Pair;

pub fn pair_to_ty_lambda(p: Pair<'_, Rule>) -> Result<LambdaSub<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Super Type", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let sup_rule = inner.remove(0);
    let super_ty = pair_to_type(sup_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(LambdaSub::new(var, super_ty, term))
}

pub fn pair_to_ty_lambda_unbounded(p: Pair<'_, Rule>) -> Result<LambdaSub<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let annot_rule = inner.remove(0);
    let kind = pair_to_kind(annot_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(LambdaSub::new(var, Top::new(kind), term))
}

pub fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp<Term>, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Applied Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp::new(t, ty))
}
