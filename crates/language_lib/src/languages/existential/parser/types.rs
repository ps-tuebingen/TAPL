use super::{pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type};
use common::parse::{UnexpectedRule, UnknownKeyword};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::{
    kinds::Kind,
    types::{Bool, Exists, Fun, Nat, Record, TypeVariable, Unit},
};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Type"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_prim_ty(prim_inner)?;

    let ty = match inner.next() {
        None => prim_ty,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_ty(leftrec_inner, prim_ty)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }
    Ok(ty)
}

fn pair_to_prim_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::paren_type => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        Rule::const_ty => str_to_type(p.as_str()),
        Rule::pack_ty => pair_to_pack_ty(p),
        Rule::record_ty => pair_to_record_ty(p),
        Rule::variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
        _ => Err(UnexpectedRule::new(p, "Non Left-Recursive Type").into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_ty(p, ty),
        _ => Err(UnexpectedRule::new(p, "Left Recursive Term").into()),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        "unit" => Ok(Unit::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}

fn pair_to_pack_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
    let start_rule = inner.remove(0);
    let mut start_inner = pair_to_n_inner(start_rule, vec!["Exists Keyword", "Exists Variable"])?;
    start_inner.remove(0);
    let var = start_inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let inner_ty = pair_to_type(ty_rule)?;
    Ok(Exists::new(var, Kind::Star, inner_ty).into())
}

fn pair_to_record_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let label = var_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(MissingInput::new("Record Type"))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(label, ty);
    }
    Ok(Record::new(records).into())
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Function To Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Fun::new(ty, to_ty).into())
}
