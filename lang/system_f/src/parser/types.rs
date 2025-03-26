use super::{pair_to_n_inner, Error, Rule};
use crate::types::Type;
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursie Type".to_owned()))?;
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
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(ty)
}

fn pair_to_prim_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::forall_ty => pair_to_forall(p),
        Rule::variable => Ok(Type::Var(p.as_str().trim().to_owned())),
        Rule::paren_ty => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        r => Err(Error::unexpected(r, "Forall Type or Type Variable")),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_type(p, ty),
        r => Err(Error::unexpected(r, "Function Type")),
    }
}

fn pair_to_forall(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Keyword", "Forall Variable", "Forall Type"])?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim().to_owned();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Type::Forall(var, Box::new(ty)))
}

fn pair_to_fun_type(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(ty_rule)?;
    Ok(Type::Fun(Box::new(ty), Box::new(to_ty)))
}
