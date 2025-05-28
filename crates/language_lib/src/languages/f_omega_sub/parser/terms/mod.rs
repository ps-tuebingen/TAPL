use super::{pair_to_kind, pair_to_n_inner, pair_to_type, to_parse_err, Error, Rule, Term};
use common::errors::ErrorKind;
use pest::iterators::Pair;
use syntax::terms::{App, Num, RecordProj, Variable};

mod lambda;
mod lett;
mod nat;
mod pack;
mod record;
mod ty_lambda;
use lambda::pair_to_lambda;
use lett::pair_to_let;
use nat::{pair_to_pred, pair_to_succ};
use pack::{pair_to_pack, pair_to_unpack};
use record::pair_to_record;
use ty_lambda::{pair_to_ty_lambda, pair_to_ty_lambda_unbounded, pair_to_tyapp};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Non Left-Recursive Term".to_owned(),
    )))?;
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
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
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
        Rule::let_term => pair_to_let(p).map(|lett| lett.into()),
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::pred_term => pair_to_pred(p).map(|p| p.into()),
        Rule::tylambda_term => pair_to_ty_lambda(p).map(|tylam| tylam.into()),
        Rule::tylambda_unbounded => pair_to_ty_lambda_unbounded(p).map(|tylam| tylam.into()),
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
                .map_err(|_| to_parse_err(ErrorKind::UnknownKeyword(p.as_str().to_owned())))?;
            Ok(Num::new(num).into())
        }
        _ => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{p:?}"),
            expected: "Non Left-Recursive Term".to_owned(),
        })),
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
        _ => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{p:?}"),
            expected: "Left Recursive Term".to_owned(),
        })),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "zero" => Ok(Num::new(0).into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}
