use super::{get_n_inner, next_rule, to_parse_err, Rule};
use crate::terms::Term;
use common::{
    errors::{Error, ErrorKind},
    terms::{App, False, Num, True, Unit, Variable},
};
use pest::iterators::Pair;

mod ascribe;
mod case;
mod fix;
mod ift;
mod lambda;
mod lett;
mod list;
mod nat;
mod optional;
mod proj;
mod record;
mod sum;
mod tup;
mod variant;
use ascribe::pair_to_ascribe;
use case::pair_to_case;
use fix::pair_to_fix;
use ift::pair_to_if;
use lambda::pair_to_lambda;
use lett::pair_to_let;
use list::{pair_to_cons, pair_to_head, pair_to_isnil, pair_to_nil, pair_to_tail};
use nat::{pair_to_isz, pair_to_num, pair_to_pred, pair_to_succ};
use optional::{pair_to_none, pair_to_some};
use proj::pair_to_proj;
use record::pair_to_rec;
use sum::{pair_to_left, pair_to_right};
use tup::pair_to_tup;
use variant::pair_to_variant;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    if p.as_rule() != Rule::term {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{:?}", p.as_rule()),
            expected: "Term".to_owned(),
        }));
    }

    let mut inner = p.into_inner();
    let prim_term_pair = inner
        .next()
        .ok_or(to_parse_err(ErrorKind::MissingInput("Term".to_owned())))?;
    let prim_term_rule = get_n_inner(prim_term_pair, vec!["Prim Term"])?.remove(0);
    let prim_term = pair_to_primterm(prim_term_rule)?;

    let term = if let Some(p) = inner.next() {
        pair_to_leftrec(p, prim_term)?
    } else {
        prim_term
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(term)
}

pub fn pair_to_primterm(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::r#const => const_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::some_term => pair_to_some(p).map(|som| som.into()),
        Rule::none_term => pair_to_none(p).map(|non| non.into()),
        Rule::fix_term => pair_to_fix(p).map(|fix| fix.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::let_term => pair_to_let(p).map(|lett| lett.into()),
        Rule::tup_term => pair_to_tup(p).map(|tup| tup.into()),
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
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "NOn Left-Recursive Term".to_owned(),
        })),
    }
}

fn const_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        "zero" => Ok(Num::new(0).into()),
        "unit" => Ok(Unit::new().into()),
        _ => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn paren_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let term_rule = get_n_inner(p, vec!["Term"])?.remove(0);
    pair_to_term(term_rule)
}

pub fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::left_rec => {
            let next = get_n_inner(p, vec!["Left Recursive Term"])?.remove(0);
            match next.as_rule() {
                Rule::ascription => {
                    let pair = next_rule(next, Rule::ascription)?;
                    pair_to_ascribe(pair, t).map(|asc| asc.into())
                }
                Rule::proj => {
                    let pair = next_rule(next, Rule::proj)?;
                    pair_to_proj(pair, t)
                }
                Rule::term => {
                    let arg = pair_to_term(next)?;
                    Ok(App {
                        fun: Box::new(t),
                        arg: Box::new(arg),
                    }
                    .into())
                }
                r => Err(to_parse_err(ErrorKind::UnexpectedRule {
                    found: format!("{r:?}"),
                    expected: "Ascription, Projection or Term".to_owned(),
                })),
            }
        }
        Rule::EOI => Ok(t),
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Left Recursive Term or End of Input".to_owned(),
        })),
    }
}
