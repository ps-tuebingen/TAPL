use super::{pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type};
use common::parse::UnexpectedRule;
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    types::{Forall, Fun, TypeVariable},
};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursie Type"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_prim_type(prim_inner)?;

    let ty = match inner.next() {
        None => prim_ty,
        Some(left_rec) => {
            let left_rec_inner = pair_to_n_inner(left_rec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_ty(left_rec_inner, prim_ty)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }
    Ok(ty)
}

fn pair_to_prim_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::forall_ty => pair_to_forall(p),
        Rule::variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
        Rule::paren_ty => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        r => Err(UnexpectedRule::new(r, "Forall Type or Type Variable").into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_type(p, ty),
        r => Err(UnexpectedRule::new(r, "Function Type").into()),
    }
}

fn pair_to_forall(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Type"])?;
    let start_rule = inner.remove(0);
    let mut start_inner = pair_to_n_inner(start_rule, vec!["Forall Keyword", "Forall Variable"])?;
    start_inner.remove(0);
    let var = start_inner.remove(0).as_str().trim();

    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Forall::new(var, Kind::Star, ty).into())
}

fn pair_to_fun_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(ty_rule)?;
    Ok(Fun::new(ty, to_ty).into())
}
