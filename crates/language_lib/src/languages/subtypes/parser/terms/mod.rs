use super::{
    pair_to_n_inner, pair_to_type, Error, MissingInput, RemainingInput, Rule, Term, Type,
    UnexpectedRule, UnknownKeyword,
};
use pest::iterators::Pair;
use syntax::{
    terms::{App, False, Lambda, Num, True, Unit, Variable},
    types::Unit as UnitTy,
};

mod bool;
mod cast;
mod fix;
mod lambda;
mod let_exp;
mod list;
mod nat;
mod record;
mod references;
mod variant;
use bool::pair_to_if;
use cast::pair_to_cast;
use fix::pair_to_fix;
use lambda::pair_to_lambda;
use let_exp::pair_to_let;
use list::{pair_to_cons, pair_to_listcase, pair_to_nil};
use nat::{pair_to_pred, pair_to_succ};
use record::{pair_to_proj, pair_to_record};
use references::{pair_to_assign, pair_to_deref, pair_to_ref};
use variant::{pair_to_variant, pair_to_variantcase};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non left-recursive term"))?;
    let prim_pair = pair_to_n_inner(prim_rule, vec!["Non Left-recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_pair)?;

    let term = match inner.next() {
        Some(p) => {
            let leftrec_inner = pair_to_n_inner(p, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
        None => prim_term,
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }

    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::r#const => str_to_term(p.as_str()),
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::rec_term => pair_to_record(p).map(|rec| rec.into()),
        Rule::variant_term => pair_to_variant(p).map(|variant| variant.into()),
        Rule::variant_case => pair_to_variantcase(p).map(|case| case.into()),
        Rule::nil_term => pair_to_nil(p).map(|n| n.into()),
        Rule::cons_term => pair_to_cons(p).map(|c| c.into()),
        Rule::list_case => pair_to_listcase(p).map(|case| case.into()),
        Rule::succ_term => pair_to_succ(p).map(|succ| succ.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::ref_term => pair_to_ref(p).map(|reft| reft.into()),
        Rule::deref_term => pair_to_deref(p).map(|deref| deref.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::let_term => pair_to_let(p).map(|lt| lt.into()),
        Rule::fix_term => pair_to_fix(p).map(|fix| fix.into()),
        Rule::paren_term => {
            let inner = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(inner)
        }
        r => Err(UnexpectedRule::new(r, "Non Left-recursive term").into()),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        "zero" => Ok(Num::new(0).into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::cast_term => pair_to_cast(p, t).map(|cast| cast.into()),
        Rule::proj_term => pair_to_proj(p, t).map(|proj| proj.into()),
        Rule::assign_term => pair_to_assign(p, t).map(|ass| ass.into()),
        Rule::seq_term => {
            let second = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            let term = pair_to_term(second)?;
            Ok(App::new(Lambda::new("_", UnitTy::new(), term), t).into())
        }
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App {
                fun: Box::new(t),
                arg: Box::new(arg),
            }
            .into())
        }
        r => Err(UnexpectedRule::new(r, "Left Recursive Term").into()),
    }
}
