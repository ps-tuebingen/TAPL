use super::{
    pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type, UnexpectedRule,
    UnknownKeyword,
};
use pest::iterators::Pair;
use syntax::types::{Bool, Fun, Nat, Reference, Unit};

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

fn pair_to_primtype(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_type => str_to_prim_type(p.as_str()),
        Rule::ref_type => pair_to_ref_type(p),
        Rule::paren_type => {
            let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner)
        }
        _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Type").into()),
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

fn str_to_prim_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "nat" => Ok(Nat::new().into()),
        "bool" => Ok(Bool::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}

fn pair_to_ref_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Reference::new(ty).into())
}
