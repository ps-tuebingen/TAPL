use super::{pair_to_kind, pair_to_n_inner, to_parse_err, Rule};
use crate::types::Type;
use common::{
    errors::{Error, ErrorKind},
    kinds::Kind,
    types::{Bool, Forall, Fun, Nat, OpApp, OpLambda, TypeVariable, Unit},
};
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Non Left-Recursive Type".to_owned(),
    )))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_prim_type(prim_inner)?;

    let ty = match inner.next() {
        None => prim_ty,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_ty(leftrec_inner, prim_ty)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(ty)
}

fn pair_to_prim_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_ty => str_to_ty(p.as_str()),
        Rule::forall_ty => pair_to_forall(p),
        Rule::lambda_ty => pair_to_lambda_ty(p),
        Rule::paren_type => {
            let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner)
        }
        Rule::variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Non Left-Recursive Type".to_owned(),
        })),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_ty(p, ty),
        Rule::r#type => {
            let arg_ty = pair_to_type(p)?;
            Ok(OpApp::new(ty, arg_ty).into())
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Left Recursive Type".to_owned(),
        })),
    }
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_lambda_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Kind", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let knd_rule = inner.remove(0);
    let knd = pair_to_kind(knd_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(OpLambda::new(var, knd, body).into())
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(inner)?;
    Ok(Fun::new(ty, to_ty).into())
}

fn pair_to_forall(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Type"])?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Keyword", "Forall Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(Forall::new(var, Kind::Star, ty).into())
}
