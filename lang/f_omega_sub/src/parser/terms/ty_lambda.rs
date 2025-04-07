use super::{pair_to_kind, pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::syntax::{
    terms::{Term, TyApp, TyLambda},
    types::Type,
};
use pest::iterators::Pair;

pub fn pair_to_ty_lambda(p: Pair<'_, Rule>) -> Result<TyLambda, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Super Type", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let sup_rule = inner.remove(0);
    let super_ty = pair_to_type(sup_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda {
        var,
        sup_ty: super_ty,
        body: Box::new(term),
    })
}

pub fn pair_to_ty_lambda_unbounded(p: Pair<'_, Rule>) -> Result<TyLambda, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let annot_rule = inner.remove(0);
    let kind = pair_to_kind(annot_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda {
        var,
        sup_ty: Type::Top(kind),
        body: Box::new(term),
    })
}

pub fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Applied Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp {
        term: Box::new(t),
        ty,
    })
}
