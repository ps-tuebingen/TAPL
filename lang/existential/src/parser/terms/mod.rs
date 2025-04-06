use super::{pair_to_n_inner, pair_to_type, Error, Rule};
use crate::terms::{App, False, RecordProj, Term, True, Zero};
use pest::iterators::Pair;

mod bool;
mod fix;
mod lambda;
mod nat;
mod pack;
mod record;
use bool::pair_to_if;
use fix::pair_to_fix;
use lambda::pair_to_lambda;
use nat::{pair_to_iszero, pair_to_pred, pair_to_succ};
use pack::{pair_to_pack, pair_to_unpack};
use record::pair_to_record;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::missing("Non Left-Recursive Term"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_primterm(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
    };
    Ok(term)
}

fn pair_to_primterm(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::const_term => str_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::pack_term => pair_to_pack(p).map(|pck| pck.into()),
        Rule::unpack_term => pair_to_unpack(p).map(|unp| unp.into()),
        Rule::succ_term => pair_to_succ(p).map(|succ| succ.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::iszero_term => pair_to_iszero(p).map(|isz| isz.into()),
        Rule::fix_term => pair_to_fix(p).map(|fix| fix.into()),
        Rule::record_term => pair_to_record(p).map(|rec| rec.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::unknown(p.as_str()))?;
            Ok(num.into())
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        _ => Err(Error::unexpected(&p, "Non Left-Recursive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::record_proj => {
            let label = p.as_str().trim().to_owned();
            Ok(RecordProj {
                term: Box::new(t),
                label,
            }
            .into())
        }
        Rule::term => {
            let arg_term = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg_term),
            }
            .into())
        }
        _ => Err(Error::unexpected(&p, "Left Recursive Term")),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Term::Unit),
        "zero" => Ok(Zero.into()),
        "true" => Ok(True.into()),
        "false" => Ok(False.into()),
        s => Err(Error::unknown(s)),
    }
}
