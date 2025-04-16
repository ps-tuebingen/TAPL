use super::{pair_to_n_inner, pair_to_type, to_parse_err, Rule};
use crate::terms::Term;
use common::{
    errors::{Error, ErrorKind},
    terms::{App, Num, Variable},
};
use pest::iterators::Pair;

mod lambda;
mod lambda_sub;
mod nat;
mod pack;
mod record;
use lambda::pair_to_lambda;
use lambda_sub::{pair_to_lambda_sub, pair_to_tyapp, pair_to_tylambda};
use nat::{pair_to_pred, pair_to_succ};
use pack::{pair_to_pack, pair_to_unpack};
use record::{pair_to_proj, pair_to_rec};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(ErrorKind::MissingInput(
            "Non Left-Recursive Term".to_owned(),
        ))
        .map_err(to_parse_err)?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::lambda_sub => pair_to_lambda_sub(p).map(|lam| lam.into()),
        Rule::ty_lambda => pair_to_tylambda(p).map(|lam| lam.into()),
        Rule::pack_term => pair_to_pack(p).map(|pack| pack.into()),
        Rule::unpack_term => pair_to_unpack(p).map(|unp| unp.into()),
        Rule::rec_term => pair_to_rec(p).map(|rec| rec.into()),
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::pred_term => pair_to_pred(p).map(|p| p.into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| ErrorKind::UnknownKeyword(p.as_str().to_owned()))
                .map_err(to_parse_err)?;
            Ok(Num::new(num).into())
        }
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Non Left-Recursive Term".to_owned(),
        })),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::proj => pair_to_proj(p, t).map(|proj| proj.into()),
        Rule::tyapp => pair_to_tyapp(p, t).map(|app| app.into()),
        Rule::paren_tyapp => {
            let inner = pair_to_n_inner(p, vec!["Type Application"])?.remove(0);
            pair_to_leftrec(inner, t)
        }
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Type or Term Application".to_owned(),
        })),
    }
}
