use crate::{
    parser::{errors::Error, get_n_inner, next_rule, Rule},
    types::Type,
};
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::prim_type => str_to_ty(p.as_str()),
        Rule::fun_type => pair_to_fun_type(p),
        Rule::prod_type => pair_to_prod_type(p),
        Rule::record_type => pair_to_rec_type(p),
        Rule::sum_type => pair_to_sum_type(p),
        Rule::variant_type => pair_to_variant_type(p),
        Rule::tuple_type => pair_to_tuple_type(p),
        r => Err(Error::BadRule(r)),
    }
}

fn pair_to_fun_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = get_n_inner(p, vec!["From Type", "To Type"])?;

    let from_pair = inner.remove(0);
    let from_rule = next_rule(from_pair, Rule::r#type)?;
    let from_ty = pair_to_type(from_rule)?;

    let to_pair = inner.remove(0);
    let to_rule = next_rule(to_pair, Rule::r#type)?;
    let to_ty = pair_to_type(to_rule)?;

    Ok(Type::Fun(Box::new(from_ty), Box::new(to_ty)))
}

fn pair_to_prod_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = get_n_inner(p, vec!["Product Type First", "Product Type Second"])?;
    let fst_pair = inner.remove(0);
    let fst_rule = next_rule(fst_pair, Rule::r#type)?;
    let fst_ty = pair_to_type(fst_rule)?;

    let snd_pair = inner.remove(0);
    let snd_rule = next_rule(snd_pair, Rule::r#type)?;
    let snd_ty = pair_to_type(snd_rule)?;

    Ok(Type::Prod(Box::new(fst_ty), Box::new(snd_ty)))
}

fn pair_to_rec_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut recs = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(n) = inner.next() {
        let next_var = n.as_str().to_owned();
        let next_pair = inner
            .next()
            .ok_or(Error::MissingInput("Record Type".to_owned()))?;
        let next_rule = next_rule(next_pair, Rule::r#type)?;
        let next_ty = pair_to_type(next_rule)?;
        recs.insert(next_var, next_ty);
    }
    Ok(Type::Record(recs))
}

fn pair_to_sum_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = get_n_inner(p, vec!["First Sum Type", "Second Sum Type"])?;

    let fst_pair = inner.remove(0);
    let fst_rule = next_rule(fst_pair, Rule::r#type)?;
    let fst_ty = pair_to_type(fst_rule)?;

    let snd_pair = inner.remove(0);
    let snd_rule = next_rule(snd_pair, Rule::r#type)?;
    let snd_ty = pair_to_type(snd_rule)?;

    Ok(Type::Sum(Box::new(fst_ty), Box::new(snd_ty)))
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut variants = HashMap::new();
    while let Some(n) = inner.next() {
        let label = n.as_str().to_owned();
        let next_pair = inner
            .next()
            .ok_or(Error::MissingInput("Variant Type".to_owned()))?;
        let n_rule = next_rule(next_pair, Rule::r#type)?;
        let n_ty = pair_to_type(n_rule)?;
        variants.insert(label, n_ty);
    }
    Ok(Type::Variant(variants))
}

fn pair_to_tuple_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut tys = vec![];
    for inner_pair in p.into_inner() {
        let inner_rule = next_rule(inner_pair, Rule::r#type)?;
        let inner_ty = pair_to_type(inner_rule)?;
        tys.push(inner_ty)
    }
    Ok(Type::Tup(tys))
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "bool" => Ok(Type::Bool),
        "nat" => Ok(Type::Nat),
        "unit" => Ok(Type::Unit),
        _ => Err(Error::BadType(s.to_owned())),
    }
}
