use super::{pair_to_n_inner, to_parse_err, Rule, Type};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::{
    Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Unit, Variant,
};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::prim_type => str_to_type(p.as_str()),
        Rule::fun_type => pair_to_fun_type(p),
        Rule::rec_type => pair_to_rec_type(p),
        Rule::variant_type => pair_to_variant_type(p),
        Rule::list_type => pair_to_list_type(p),
        Rule::ref_type => pair_to_ref_type(p),
        Rule::sink_type => pair_to_sink_type(p),
        Rule::source_type => pair_to_source_type(p),
        Rule::paren_ty => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
            pair_to_type(ty_pair)
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Type".to_owned(),
        })),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "top" | "top[*]" => Ok(Top::new_star().into()),
        "bot" => Ok(Bot.into()),
        "nat" => Ok(Nat::new().into()),
        "unit" => Ok(Unit::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_fun_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["From Type", "To Type"])?;
    let from_rule = inner.remove(0);
    let from_pair = pair_to_n_inner(from_rule, vec!["Type"])?.remove(0);
    let from_ty = pair_to_type(from_pair)?;
    let to_rule = inner.remove(0);
    let to_pair = pair_to_n_inner(to_rule, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(to_pair)?;
    Ok(Fun::new(from_ty, to_ty).into())
}

fn pair_to_rec_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut records = HashMap::new();
    while let Some(next) = inner.next() {
        let var = next.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Record Type".to_owned(),
        )))?;
        let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
        let ty = pair_to_type(ty_pair)?;
        records.insert(var, ty);
    }

    Ok(Record::new(records).into())
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut variants = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(next) = inner.next() {
        let label = next.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Variant Type".to_owned(),
        )))?;
        let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
        let ty = pair_to_type(ty_pair)?;
        variants.insert(label, ty);
    }

    Ok(Variant::new(variants).into())
}

fn pair_to_list_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["List Keyword", "List Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(List::new(ty).into())
}

fn pair_to_ref_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Reference::new(ty).into())
}

fn pair_to_sink_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Sink Keyword", "Sink Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Sink::new(ty).into())
}

fn pair_to_source_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Source Keyword", "Source Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Source::new(ty).into())
}
