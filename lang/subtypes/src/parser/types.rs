use super::{pair_to_n_inner, to_parse_err, Rule};
use crate::types::Type;
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use std::collections::HashMap;

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
        "top" => Ok(Type::Top),
        "bot" => Ok(Type::Bot),
        "nat" => Ok(Type::Nat),
        "unit" => Ok(Type::Unit),
        "bool" => Ok(Type::Bool),
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
    Ok(Type::Fun {
        from: Box::new(from_ty),
        to: Box::new(to_ty),
    })
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

    Ok(Type::Record(records))
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut variants = vec![];
    let mut inner = p.into_inner();
    while let Some(next) = inner.next() {
        let label = next.as_str().trim().to_owned();
        let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
            "Variant Type".to_owned(),
        )))?;
        let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
        let ty = pair_to_type(ty_pair)?;
        variants.push((label, ty));
    }

    Ok(Type::Variant(variants))
}

fn pair_to_list_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["List Keyword", "List Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Type::List(Box::new(ty)))
}

fn pair_to_ref_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Type::Ref(Box::new(ty)))
}

fn pair_to_sink_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Sink Keyword", "Sink Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Type::Sink(Box::new(ty)))
}

fn pair_to_source_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Source Keyword", "Source Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Type::Source(Box::new(ty)))
}
