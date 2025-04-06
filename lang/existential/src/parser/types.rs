use super::{pair_to_n_inner, Error, Rule};
use crate::types::Type;
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::missing("Non Left-Recursive Type"))?;
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
        return Err(Error::remaining(&n));
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
        Rule::variable => Ok(Type::Var(p.as_str().trim().to_owned())),
        _ => Err(Error::unexpected(&p, "Non Left-Recursive Type")),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_ty(p, ty),
        _ => Err(Error::unexpected(&p, "Left Recursive Type")),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "nat" => Ok(Type::Nat),
        "bool" => Ok(Type::Bool),
        "unit" => Ok(Type::Unit),
        s => Err(Error::unknown(s)),
    }
}

fn pair_to_pack_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
    let start_rule = inner.remove(0);
    let mut start_inner = pair_to_n_inner(start_rule, vec!["Exists Keyword", "Exists Variable"])?;
    start_inner.remove(0);
    let var = start_inner.remove(0).as_str().trim().to_owned();
    let ty_rule = inner.remove(0);
    let inner_ty = pair_to_type(ty_rule)?;
    Ok(Type::Package {
        ty_var: var,
        ty: Box::new(inner_ty),
    })
}

fn pair_to_record_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(var_rule) = inner.next() {
        let label = var_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(Error::missing("Record Type"))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(label, ty);
    }
    Ok(Type::Record(records))
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Function To Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Type::Fun {
        from: Box::new(ty),
        to: Box::new(to_ty),
    })
}
