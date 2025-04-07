use super::{pair_to_kind, pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::syntax::{
    kinds::Kind,
    terms::{Term, TyApp, TyLambda},
};
use pest::iterators::Pair;

pub fn pair_to_ty_lambda(p: Pair<'_, Rule>) -> Result<TyLambda, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["TyLambda Variable", "TyLambda Kind", "TyLambda Term"],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let kind_rule = inner.remove(0);
    let kind = pair_to_kind(kind_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda {
        var,
        annot: kind,
        body: Box::new(term),
    })
}

pub fn pair_to_ty_lambda_star(p: Pair<'_, Rule>) -> Result<TyLambda, Error> {
    let mut inner = pair_to_n_inner(p, vec!["TyLambda Variable", "TyLambda Term"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(TyLambda {
        var,
        annot: Kind::Star,
        body: Box::new(term),
    })
}

pub fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Applied Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp {
        fun: Box::new(t),
        arg: ty,
    })
}
