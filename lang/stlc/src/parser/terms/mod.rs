use super::{errors::Error, get_n_inner, next_rule, Rule};
use crate::syntax::{False, Term, True, Unit, Zero};
use pest::iterators::Pair;

mod ascribe;
mod case;
mod ift;
mod lambda;
mod lett;
mod list;
mod nat;
mod proj;
mod record;
mod sum;
mod tup;
mod variant;
use ascribe::pair_to_ascribe;
use case::pair_to_case;
use ift::pair_to_if;
use lambda::{pair_to_app, pair_to_lambda};
use lett::pair_to_let;
use list::{pair_to_cons, pair_to_head, pair_to_isnil, pair_to_nil, pair_to_tail};
use nat::{pair_to_isz, pair_to_num, pair_to_pred, pair_to_succ};
use proj::pair_to_proj;
use record::pair_to_rec;
use sum::{pair_to_left, pair_to_right};
use tup::pair_to_tup;
use variant::pair_to_variant;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        Rule::r#const => const_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::app_term => pair_to_app(p).map(|app| app.into()),
        Rule::ascription => pair_to_ascribe(p).map(|asc| asc.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::let_term => pair_to_let(p).map(|lett| lett.into()),
        Rule::tup_term => pair_to_tup(p).map(|tup| tup.into()),
        Rule::proj => pair_to_proj(p).map(|proj| proj.into()),
        Rule::rec_term => pair_to_rec(p).map(|rec| rec.into()),
        Rule::left_term => pair_to_left(p).map(|lft| lft.into()),
        Rule::right_term => pair_to_right(p).map(|rgt| rgt.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::variant_term => pair_to_variant(p).map(|v| v.into()),
        Rule::succ_term => pair_to_succ(p).map(|s| s.into()),
        Rule::isz_term => pair_to_isz(p).map(|isz| isz.into()),
        Rule::cons_term => pair_to_cons(p).map(|cons| cons.into()),
        Rule::nil_term => pair_to_nil(p).map(|nil| nil.into()),
        Rule::isnil_term => pair_to_isnil(p).map(|isn| isn.into()),
        Rule::head_term => pair_to_head(p).map(|hd| hd.into()),
        Rule::tail_term => pair_to_tail(p).map(|tl| tl.into()),
        Rule::number => pair_to_num(p),
        Rule::case_term => pair_to_case(p),
        Rule::paren_term => paren_to_term(p),
        r => Err(Error::BadRule(r)),
    }
}

fn const_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True.into()),
        "false" => Ok(False.into()),
        "zero" => Ok(Zero.into()),
        "unit" => Ok(Unit.into()),
        _ => Err(Error::BadTerm(s.to_owned())),
    }
}

fn paren_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let term_rule = get_n_inner(p, vec!["Term"])?.remove(0);
    let next_rule = next_rule(term_rule, Rule::term)?;
    pair_to_term(next_rule)
}
