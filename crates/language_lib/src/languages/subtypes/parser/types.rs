use super::{
    pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type, UnexpectedRule,
    UnknownKeyword,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::{
    Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Unit, Variant,
};
pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Type"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_primtype(prim_inner)?;

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

pub fn pair_to_primtype(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_type => str_to_type(p.as_str()),
        Rule::top_type_star | Rule::top_type => Ok(Top::new_star().into()),
        Rule::record_type => pair_to_rec_type(p),
        Rule::variant_type => pair_to_variant_type(p),
        Rule::list_type => pair_to_list_type(p),
        Rule::ref_type => pair_to_ref_type(p),
        Rule::sink_type => pair_to_sink_type(p),
        Rule::source_type => pair_to_source_type(p),
        Rule::paren_type => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        r => Err(UnexpectedRule::new(r, &format!("Non Left-Recursive Type ({:?})", p)).into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_type => {
            let to_rule = pair_to_n_inner(p, vec!["Function To Type"])?.remove(0);
            let to_ty = pair_to_type(to_rule)?;
            Ok(Fun {
                from: Box::new(ty),
                to: Box::new(to_ty),
            }
            .into())
        }
        _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Type").into()),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "bot" => Ok(Bot::new().into()),
        "nat" => Ok(Nat::new().into()),
        "unit" => Ok(Unit::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}

fn pair_to_rec_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(next) = inner.next() {
        let var = next.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(MissingInput::new("Record Type"))?;
        let ty = pair_to_type(ty_rule)?;
        records.insert(var, ty);
    }

    Ok(Record::new(records).into())
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut variants = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(next) = inner.next() {
        let label = next.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(MissingInput::new("Variant Type"))?;
        let ty = pair_to_type(ty_rule)?;
        variants.insert(label, ty);
    }

    Ok(Variant::new(variants).into())
}

fn pair_to_list_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["List Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(List::new(ty).into())
}

fn pair_to_ref_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Reference::new(ty).into())
}

fn pair_to_sink_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Sink Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Sink::new(ty).into())
}

fn pair_to_source_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Source Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Source::new(ty).into())
}
