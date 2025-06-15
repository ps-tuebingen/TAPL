use super::{
    get_n_inner, Error, MissingInput, RemainingInput, Rule, Type, UnexpectedRule, UnknownKeyword,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::{Bool, Fun, Nat, Product, Record, Sum, Tuple, Unit, Variant};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Type"))?;
    let prim_inner = get_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_primtype(prim_inner)?;

    let ty = match inner.next() {
        None => prim_ty,
        Some(leftrec) => {
            let leftrec_inner = get_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
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
        Rule::paren_type => {
            let inner = get_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner)
        }
        Rule::const_type => str_to_ty(p.as_str()),
        Rule::prod_type => pair_to_prod_type(p),
        Rule::record_type => pair_to_rec_type(p),
        Rule::sum_type => pair_to_sum_type(p),
        Rule::variant_type => pair_to_variant_type(p),
        Rule::tuple_type => pair_to_tuple_type(p),
        r => Err(UnexpectedRule::new(r, "Non Left-Recursive Type").into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_type => {
            let to_rule = get_n_inner(p, vec!["Function To Type"])?.remove(0);
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

fn pair_to_prod_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = get_n_inner(p, vec!["Product Type First", "Product Type Second"])?;
    let fst_pair = inner.remove(0);
    let fst_ty = pair_to_type(fst_pair)?;

    let snd_pair = inner.remove(0);
    let snd_ty = pair_to_type(snd_pair)?;

    Ok(Product::new(fst_ty, snd_ty).into())
}

fn pair_to_rec_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut recs = HashMap::new();
    let mut inner = p.into_inner();
    while let Some(n) = inner.next() {
        let next_var = n.as_str().to_owned();
        let next_pair = inner.next().ok_or(MissingInput::new("Record Type"))?;
        let next_ty = pair_to_type(next_pair)?;
        recs.insert(next_var, next_ty);
    }
    Ok(Record::new(recs).into())
}

fn pair_to_sum_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = get_n_inner(p, vec!["First Sum Type", "Second Sum Type"])?;

    let fst_pair = inner.remove(0);
    let fst_ty = pair_to_type(fst_pair)?;

    let snd_pair = inner.remove(0);
    let snd_ty = pair_to_type(snd_pair)?;

    Ok(Sum::new(fst_ty, snd_ty).into())
}

fn pair_to_variant_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let mut variants = HashMap::new();
    while let Some(n) = inner.next() {
        let label = n.as_str().to_owned();
        let next_pair = inner.next().ok_or(MissingInput::new("Variant Type"))?;
        let n_ty = pair_to_type(next_pair)?;
        variants.insert(label, n_ty);
    }
    Ok(Variant::new(variants).into())
}

fn pair_to_tuple_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut tys = vec![];
    for inner_pair in p.into_inner() {
        let inner_ty = pair_to_type(inner_pair)?;
        tys.push(inner_ty)
    }
    Ok(Tuple::new(tys).into())
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "bool" => Ok(Bool::new().into()),
        "nat" => Ok(Nat::new().into()),
        "unit" => Ok(Unit::new().into()),
        _ => Err(UnknownKeyword::new(s).into()),
    }
}
