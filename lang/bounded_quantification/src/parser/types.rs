use super::{pair_to_n_inner, Error, Rule};
use crate::types::Type;
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Type".to_owned()))?;
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
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(ty)
}

fn pair_to_prim_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_ty => str_to_type(p.as_str()),
        Rule::forall_ty => pair_to_forall(p),
        Rule::exists_ty => pair_to_exists(p),
        Rule::record_ty => pair_to_rec_ty(p),
        Rule::paren_type => {
            let inner_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner_rule)
        }
        Rule::variable => Ok(Type::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recursive Type")),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_ty(p, ty),
        r => Err(Error::unexpected(r, "Function Type")),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "nat" => Ok(Type::Nat),
        "top" => Ok(Type::Top),
        s => Err(Error::UnknownKw(s.to_owned())),
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
    let var = var_inner.remove(0).as_str().trim().to_owned();
    let super_rule = inner.remove(0);
    let super_ty = pair_to_type(super_rule)?;

    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;

    Ok(Type::Forall {
        var,
        sup_ty: Box::new(super_ty),
        ty: Box::new(body_ty),
    })
}

fn pair_to_exists(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Exists Variable", "Exists Super Type", "Exists Type"],
    )?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Keyword", "Exists Variable"])?;
    var_inner.remove(0);
    let var = var_inner.remove(0).as_str().trim().to_owned();

    let super_rule = inner.remove(0);
    let sup_ty = pair_to_type(super_rule)?;

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    Ok(Type::Exists {
        var,
        sup_ty: Box::new(sup_ty),
        ty: Box::new(ty),
    })
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Type::Fun {
        from: Box::new(ty),
        to: Box::new(to_ty),
    })
}

fn pair_to_rec_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut recs = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(label_rule) = inner.next() {
        let label = label_rule.as_str().trim().to_owned();
        let ty_rule = inner
            .next()
            .ok_or(Error::MissingInput("Record Type".to_owned()))?;
        let ty = pair_to_type(ty_rule)?;
        recs.insert(label, ty);
    }
    Ok(Type::Record(recs))
}
