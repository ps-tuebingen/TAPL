use super::{pair_to_n_inner, to_parse_err, Error, Rule};
use crate::types::Type;
use common::{
    errors::ErrorKind,
    types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeVariable},
};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(ErrorKind::MissingInput(
            "Non Left-Recursive Type".to_owned(),
        ))
        .map_err(to_parse_err)?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_type = pair_to_prim_ty(prim_inner)?;

    let ty = match inner.next() {
        None => prim_type,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_ty(leftrec_inner, prim_type)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
    }
    Ok(ty)
}

fn pair_to_prim_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_ty => str_to_type(p.as_str()),
        Rule::forall_ty => pair_to_forall(p),
        Rule::forall_unbounded => pair_to_forall_unbounded(p),
        Rule::exists_unbounded => pair_to_exists_unbounded(p),
        Rule::exists_ty => pair_to_exists(p),
        Rule::record_ty => pair_to_rec_ty(p),
        Rule::paren_type => {
            let inner_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner_rule)
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
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Function Type".to_owned(),
        })),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "nat" => Ok(Nat.into()),
        "top" => Ok(Top::new_star().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_forall(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Forall Variable", "Forall Super Type", "Forall Body"],
    )?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Keyword", "Forall Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim();
    let super_rule = inner.remove(0);
    let super_ty = pair_to_type(super_rule)?;

    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;

    Ok(ForallBounded::new(var, super_ty, body_ty).into())
}

fn pair_to_forall_unbounded(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Keyword", "Forall Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;
    Ok(ForallBounded::new_unbounded(var, body_ty).into())
}

fn pair_to_exists(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Exists Variable", "Exists Super Type", "Exists Type"],
    )?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Keyword", "Exists Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim();

    let super_rule = inner.remove(0);
    let sup_ty = pair_to_type(super_rule)?;

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(ExistsBounded::new(var, sup_ty, ty).into())
}

fn pair_to_exists_unbounded(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Keyword", "Exists Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;

    Ok(ExistsBounded::new_unbounded(var, body_ty).into())
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Fun::new(ty, to_ty).into())
}

fn pair_to_rec_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut recs = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(label_rule) = inner.next() {
        let label = label_rule.as_str().trim().to_owned();
        let ty_rule = inner
            .next()
            .ok_or(ErrorKind::MissingInput("Record Type".to_owned()))
            .map_err(to_parse_err)?;
        let ty = pair_to_type(ty_rule)?;
        recs.insert(label, ty);
    }
    Ok(Record::new(recs).into())
}
