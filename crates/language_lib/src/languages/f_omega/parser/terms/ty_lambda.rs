use super::{pair_to_kind, pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    terms::{TyApp, TyLambda},
};

pub fn pair_to_ty_lambda(p: Pair<'_, Rule>) -> Result<TyLambda<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda::new(var, kind, term))
}

pub fn pair_to_ty_lambda_star(p: Pair<'_, Rule>) -> Result<TyLambda<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["TyLambda Variable", "TyLambda Term"])?;
    let var = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda::new(var, Kind::Star, term))
}

pub fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp<Term, Type>, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Applied Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp::new(t, ty))
}
