use super::{pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type, UnexpectedRule};
use parse::Parse;
use pest::iterators::Pair;
use syntax::types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeVariable};

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Type"))?;
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
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }
    Ok(ty)
}

fn pair_to_prim_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_type => match p.as_str().to_lowercase().trim() {
            "nat" => Ok(Nat::new().into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Nat").into()),
        },
        Rule::top_type_star | Rule::top_type => Ok(Top::new_star().into()),
        Rule::forall_bounded_type => Ok(ForallBounded::from_pair(p)?.into()),
        Rule::forall_unbounded_type => pair_to_forall_unbounded(p),
        Rule::exists_unbounded_type => pair_to_exists_unbounded(p),
        Rule::exists_bounded_type => Ok(ExistsBounded::from_pair(p)?.into()),
        Rule::record_type => Ok(Record::from_pair(p)?.into()),
        Rule::paren_type => {
            let inner_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner_rule)
        }
        Rule::variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
        r => Err(UnexpectedRule::new(r, "Non Left-Recursive Type").into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_type => pair_to_fun_ty(p, ty),
        r => Err(UnexpectedRule::new(r, "Function Type").into()),
    }
}

fn pair_to_forall_unbounded(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Body"])?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Forall Variable"])?;
    let var = var_inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;
    Ok(ForallBounded::new_unbounded(var, body_ty).into())
}

fn pair_to_exists_unbounded(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Exists Variable", "Exists Type"])?;
    let var_rule = inner.remove(0);
    let mut var_inner = pair_to_n_inner(var_rule, vec!["Exists Variable"])?;
    let var = var_inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body_ty = pair_to_type(body_rule)?;

    Ok(ExistsBounded::new_unbounded(var, body_ty).into())
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Function Return Type"])?.remove(0);
    let to_ty = pair_to_type(to_rule)?;
    Ok(Fun::new(ty, to_ty).into())
}
