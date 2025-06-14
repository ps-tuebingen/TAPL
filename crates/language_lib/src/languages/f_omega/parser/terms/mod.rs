use super::{
    pair_to_kind, pair_to_n_inner, pair_to_type, Error, MissingInput, RemainingInput, Rule, Term,
    Type,
};
use common::parse::{UnexpectedRule, UnknownKeyword};
use pest::iterators::Pair;
use syntax::terms::{App, False, Fix, Num, RecordProj, True, Unit, Variable};

mod ift;
mod lambda;
mod nat;
mod pack;
mod record;
mod ty_lambda;
use ift::pair_to_if;
use lambda::pair_to_lambda;
use nat::{pair_to_iszero, pair_to_pred, pair_to_succ};
use pack::{pair_to_pack, pair_to_unpack};
use record::pair_to_record;
use ty_lambda::{pair_to_ty_lambda, pair_to_ty_lambda_star, pair_to_tyapp};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Term"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_primterm(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left-Recursive Term"])?.remove(0);
            pair_to_leftrec_term(leftrec_inner, prim_term)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{n:?}")).into());
    }

    Ok(term)
}

fn pair_to_primterm(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::fix_term => {
            let inner = pair_to_n_inner(p, vec!["Fix Term"])?.remove(0);
            let inner_term = pair_to_primterm(inner)?;
            Ok(Fix::new(inner_term).into())
        }
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::pred_term => pair_to_pred(p).map(|p| p.into()),
        Rule::iszero_term => pair_to_iszero(p).map(|isz| isz.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::tylambda_term => pair_to_ty_lambda(p).map(|tylam| tylam.into()),
        Rule::tylambda_star_term => pair_to_ty_lambda_star(p).map(|tylam| tylam.into()),
        Rule::pack_term => pair_to_pack(p).map(|pack| pack.into()),
        Rule::unpack_term => pair_to_unpack(p).map(|unpack| unpack.into()),
        Rule::record_term => pair_to_record(p).map(|rec| rec.into()),
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::const_term => str_to_term(p.as_str()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Term").into()),
    }
}

fn pair_to_leftrec_term(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::ty_app => pair_to_tyapp(p, t).map(|tyapp| tyapp.into()),
        Rule::record_projection => {
            let label_rule = pair_to_n_inner(p, vec!["Projection Target"])?.remove(0);
            let label = label_rule.as_str().trim();
            Ok(RecordProj::new(t, label).into())
        }
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Term").into()),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        "unit" => Ok(Unit::new().into()),
        "zero" => Ok(Num::new(0).into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}
