use super::{
    pair_to_n_inner, pair_to_type, Error, MissingInput, RemainingInput, Rule, Term, Type,
    UnexpectedRule, UnknownKeyword,
};
use pest::iterators::Pair;
use syntax::terms::{App, False, Fst, Num, Snd, True, Unit, Variable};

mod bool;
mod fix;
mod fold;
mod lambda;
mod let_exp;
mod nat;
mod pair;
mod record;
mod variant;
use bool::pair_to_if;
use fix::pair_to_fix;
use fold::{pair_to_fold, pair_to_unfold};
use lambda::pair_to_lambda;
use let_exp::pair_to_let;
use nat::{pair_to_iszero, pair_to_pred, pair_to_succ};
use pair::pair_to_pair;
use record::{pair_to_proj, pair_to_record};
use variant::{pair_to_variant, pair_to_variantcase};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner.next().ok_or(MissingInput::new("Term"))?;
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
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
    }

    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::const_term => str_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::fold_term => pair_to_fold(p).map(|fld| fld.into()),
        Rule::unfold_term => pair_to_unfold(p).map(|unfld| unfld.into()),
        Rule::pair_term => pair_to_pair(p).map(|p| p.into()),
        Rule::variant_term => pair_to_variant(p).map(|var| var.into()),
        Rule::variantcase_term => pair_to_variantcase(p).map(|case| case.into()),
        Rule::succ_term => pair_to_succ(p).map(|succ| succ.into()),
        Rule::pred_term => pair_to_pred(p).map(|pred| pred.into()),
        Rule::iszero_term => pair_to_iszero(p).map(|isz| isz.into()),
        Rule::if_term => pair_to_if(p).map(|ift| ift.into()),
        Rule::fix_term => pair_to_fix(p).map(|fix| fix.into()),
        Rule::let_term => pair_to_let(p).map(|lt| lt.into()),
        Rule::record_term => pair_to_record(p).map(|rec| rec.into()),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| UnknownKeyword::new(p.as_str()))?;
            Ok(Num::new(num).into())
        }
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        r => Err(UnexpectedRule::new(r, "Non Left-Recursive Term").into()),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::fst_term => Ok(Fst::new(t).into()),
        Rule::snd_term => Ok(Snd::new(t).into()),
        Rule::record_proj => pair_to_proj(p, t).map(|proj| proj.into()),
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

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "zero" => Ok(Num::new(0).into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}
