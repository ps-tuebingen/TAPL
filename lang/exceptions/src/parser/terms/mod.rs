use super::{pair_to_n_inner, to_parse_err, Rule};
use crate::syntax::{App, Term, Unit};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;

mod bool;
mod err;
mod lambda;
mod nat;
use bool::pair_to_if;
use err::{pair_to_err, pair_to_raise, pair_to_try_catch, pair_to_try_with};
use lambda::pair_to_lambda;
use nat::{pair_to_isz, pair_to_pred, pair_to_succ};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_pair = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Non left-recursive Term".to_owned(),
    )))?;
    let prim_rule = pair_to_n_inner(prim_pair, vec!["Prim Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_rule)?;

    let term = match inner.next() {
        Some(p) => {
            let left_rec_rule = pair_to_n_inner(p, vec!["Application Term"])?.remove(0);
            pair_to_leftrec(left_rec_rule, prim_term)?
        }
        None => prim_term,
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(term)
}

pub fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| to_parse_err(ErrorKind::UnknownKeyword(p.as_str().to_owned())))?;
            Ok(Term::Const(num))
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        Rule::r#const => str_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::succ_term => pair_to_succ(p).map(|succ| succ.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::iszero_term => pair_to_isz(p).map(|isz| isz.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::try_term => pair_to_try_with(p).map(|tryt| tryt.into()),
        Rule::try_catch => pair_to_try_catch(p).map(|tryt| tryt.into()),
        Rule::raise_term => pair_to_raise(p).map(|r| r.into()),
        Rule::err_term => pair_to_err(p).map(|err| err.into()),
        Rule::paren_term => {
            let inner = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(inner)
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Non Left-recursive Term".to_owned(),
        })),
    }
}

pub fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
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
            expected: "Term".to_owned(),
        })),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit.into()),
        "true" => Ok(Term::True),
        "false" => Ok(Term::False),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}
