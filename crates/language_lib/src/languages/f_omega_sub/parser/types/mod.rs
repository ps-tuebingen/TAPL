use super::{pair_to_kind, pair_to_n_inner, Error, MissingInput, RemainingInput, Rule, Type};
use common::parse::{UnexpectedRule, UnknownKeyword};
use pest::iterators::Pair;
use syntax::types::{Fun, Nat, OpApp, Top, TypeVariable};

mod existential;
mod op_lambda;
mod record;
mod universal;
use existential::{pair_to_exists, pair_to_exists_unbounded};
use op_lambda::{pair_to_op_lambda, pair_to_op_lambda_star};
use record::pair_to_rec_ty;
use universal::{pair_to_universal, pair_to_universal_unbounded};

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
        return Err(RemainingInput::new(&format!("{n:?}")));
    }

    Ok(ty)
}

fn pair_to_primtype(p: Pair<'_, Rule>) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::paren_type => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            pair_to_type(ty_rule)
        }
        Rule::const_ty => str_to_type(p.as_str()),
        Rule::top_ty => {
            let kind_rule = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
            let kind = pair_to_kind(kind_rule)?;
            Ok(Top::new(kind).into())
        }
        Rule::forall_ty => pair_to_universal(p).map(|uni| uni.into()),
        Rule::forall_unbounded => pair_to_universal_unbounded(p).map(|uni| uni.into()),
        Rule::op_lambda_star => pair_to_op_lambda_star(p).map(|oplam| oplam.into()),
        Rule::op_lambda => pair_to_op_lambda(p).map(|oplam| oplam.into()),
        Rule::exists_ty => pair_to_exists(p).map(|ex| ex.into()),
        Rule::exists_unbounded => pair_to_exists_unbounded(p).map(|ex| ex.into()),
        Rule::record_ty => pair_to_rec_ty(p).map(|rec| rec.into()),
        Rule::variable => Ok(TypeVariable::new(p.as_str().trim()).into()),
        _ => Err(UnexpectedRule::new(p, "Non Left-Recursive Type").into()),
    }
}

fn pair_to_leftrec_ty(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
    match p.as_rule() {
        Rule::fun_ty => {
            let to_rule = pair_to_n_inner(p, vec!["Function To Type"])?.remove(0);
            let to_ty = pair_to_type(to_rule)?;
            Ok(Fun {
                from: Box::new(ty),
                to: Box::new(to_ty),
            }
            .into())
        }
        Rule::op_app => {
            let arg_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            let arg = pair_to_type(arg_rule)?;
            Ok(OpApp::new(ty, arg).into())
        }
        _ => Err(UnexpectedRule::new(p, "Left Recursive Type").into()),
    }
}

fn str_to_type(s: &str) -> Result<Type, Error> {
    match s.to_lowercase().trim() {
        "nat" => Ok(Nat::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}
