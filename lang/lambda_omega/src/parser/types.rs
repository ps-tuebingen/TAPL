use super::{pair_to_kind, pair_to_n_inner, Error, Rule};
use crate::types::Type;
use pest::iterators::Pair;

pub fn pair_to_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Type".to_owned()))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Type"])?.remove(0);
    let prim_ty = pair_to_prim_type(prim_inner)?;

    let ty = match inner.next() {
        None => prim_ty,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_ty(leftrec_inner, prim_ty)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(ty)
}

fn pair_to_prim_type(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::const_ty => str_to_ty(p.as_str()),
        Rule::forall_ty => todo!(),
        Rule::lambda_ty => pair_to_lambda_ty(p),
        Rule::paren_type => {
            let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(inner)
        }
        Rule::variable => Ok(Type::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recursive Type")),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => pair_to_fun_ty(p, ty),
        Rule::r#type => {
            let arg_ty = pair_to_type(p)?;
            Ok(Type::App {
                fun: Box::new(ty),
                arg: Box::new(arg_ty),
            })
        }
        r => Err(Error::unexpected(r, "Left Recursive Type")),
    }
}

fn str_to_ty(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Type::Unit),
        "nat" => Ok(Type::Nat),
        s => Err(Error::UnknownKw(s.to_owned())),
    }
}

fn pair_to_lambda_ty(p: Pair<'_, Rule>) -> Result<Type, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Kind", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let knd_rule = inner.remove(0);
    let knd = pair_to_kind(knd_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_type(body_rule)?;
    Ok(Type::Lambda {
        var,
        annot: knd,
        body: Box::new(body),
    })
}

fn pair_to_fun_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    let inner = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let to_ty = pair_to_type(inner)?;
    Ok(Type::Fun {
        from: Box::new(ty),
        to: Box::new(to_ty),
    })
}
