use super::{errors::Error, pair_to_n_inner, pair_to_type, Rule};
use crate::syntax::{App, False, Nil, Nothing, Proj1, Proj2, Term, True, Unit, Zero};
use pest::iterators::Pair;

mod ascribe;
mod fix;
mod ift;
mod lambda;
mod lett;
mod list;
mod nat;
mod pair;
mod something;
mod sum;
use ascribe::pair_to_ascribe;
use fix::pair_to_fix;
use ift::pair_to_if;
use lambda::pair_to_lambda;
use lett::pair_to_let;
use list::{pair_to_cons, pair_to_head, pair_to_isnil, pair_to_tail};
use nat::{pair_to_iszero, pair_to_pred, pair_to_succ};
use pair::pair_to_pair;
use something::{pair_to_somecase, pair_to_something};
use sum::{pair_to_left, pair_to_right, pair_to_sumcase};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(Error::MissingInput("Non Left-Recursive Term".to_owned()))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(left_rec) => {
            let left_rec_inner = pair_to_n_inner(left_rec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(left_rec_inner, prim_term)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::r#const => str_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::succ_term => pair_to_succ(p).map(|succ| succ.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::iszero_term => pair_to_iszero(p).map(|isz| isz.into()),
        Rule::let_term => pair_to_let(p).map(|lett| lett.into()),
        Rule::pair_term => pair_to_pair(p).map(|pr| pr.into()),
        Rule::left_term => pair_to_left(p).map(|l| l.into()),
        Rule::right_term => pair_to_right(p).map(|r| r.into()),
        Rule::sumcase_term => pair_to_sumcase(p).map(|case| case.into()),
        Rule::something_term => pair_to_something(p).map(|sm| sm.into()),
        Rule::some_case => pair_to_somecase(p).map(|case| case.into()),
        Rule::fix_term => pair_to_fix(p).map(|fx| fx.into()),
        Rule::cons_term => pair_to_cons(p).map(|cons| cons.into()),
        Rule::isnil_term => pair_to_isnil(p).map(|isn| isn.into()),
        Rule::head_term => pair_to_head(p).map(|hd| hd.into()),
        Rule::tail_term => pair_to_tail(p).map(|tl| tl.into()),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::NotANumber(p.as_str().to_owned()))?;
            Ok(num.into())
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-recursive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::ascribe_term => pair_to_ascribe(p, t).map(|asc| asc.into()),
        Rule::pair_fst => Ok(Proj1 { pair: Box::new(t) }.into()),
        Rule::pair_snd => Ok(Proj2 { pair: Box::new(t) }.into()),
        Rule::term => {
            let arg_term = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg_term),
            }
            .into())
        }
        r => Err(Error::unexpected(r, "Left Recursive Term")),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit.into()),
        "true" => Ok(True.into()),
        "false" => Ok(False.into()),
        "zero" => Ok(Zero.into()),
        "nothing" => Ok(Nothing.into()),
        "nil" => Ok(Nil.into()),
        s => Err(Error::UnknownKw(s.to_owned())),
    }
}
