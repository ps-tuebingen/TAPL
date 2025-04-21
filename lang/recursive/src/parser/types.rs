use super::{pair_to_n_inner, to_parse_err, Rule};
use crate::types::Type;
use common::{
    errors::{Error, ErrorKind},
    types::{Bool, Fun, Mu, Nat, Product, Record, TypeVariable, Unit, Variant},
};
use pest::iterators::Pair;
use std::collections::HashMap;

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
            pair_to_leftrec_type(leftrec_inner, prim_ty)?
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
        Rule::const_type => str_to_type(p.as_str()),
        Rule::mu_type => pair_to_mu_type(p),
        Rule::pair_type => pair_to_pair_type(p),
        Rule::variant_type => pair_to_variant_type(p),
        Rule::record_type => pair_to_record_type(p),
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

fn pair_to_leftrec_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_type => pair_to_fun_type(p, ty),
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Left Recursive Term".to_owned(),
        })),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_mu_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Mu Keyword", "Mu Variable", "Mu Body"])?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Mu::new(var, ty).into())
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut variants = HashMap::new();
    while let Some(label_rule) = inner.next() {
        let label = label_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Variant Type".to_owned(),
        )))?;
        let ty = pair_to_type(ty_rule)?;
        variants.insert(label, ty);
    }
    Ok(Variant::new(variants).into())
}

fn pair_to_record_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(label_rule) = inner.next() {
        let label = label_rule.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Record Type".to_owned(),
        )))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(label, ty);
    }
    Ok(Record::new(records).into())
}

fn pair_to_fun_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
    let to_ty = pair_to_type(inner)?;
    Ok(Fun::new(ty, to_ty).into())
}

fn pair_to_pair_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pair First Type", "Pair Second Type"])?;
    let fst_rule = inner.remove(0);
    let fst = pair_to_type(fst_rule)?;
    let snd_rule = inner.remove(0);
    let snd = pair_to_type(snd_rule)?;
    Ok(Product::new(fst, snd).into())
}
