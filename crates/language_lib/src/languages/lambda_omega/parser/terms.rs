use super::{
    pair_to_kind, pair_to_n_inner, pair_to_type, Error, MissingInput, RemainingInput, Rule, Term,
};
use common::parse::{UnexpectedRule, UnknownKeyword};
use pest::iterators::Pair;
use syntax::terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Term"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let prim_term = pair_to_prim_term(prim_inner)?;

    let term = match inner.next() {
        None => prim_term,
        Some(left_rec) => {
            let leftrec_inner = pair_to_n_inner(left_rec, vec!["Left Recursive Term"])?.remove(0);
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
        Rule::lambda_term => pair_to_lambda(p),
        Rule::tylambda_term => pair_to_tylambda(p),
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
        r => Err(UnexpectedRule::new(r, "Non Left-Recusrive Term").into()),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::tyapp => pair_to_tyapp(p, t),
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(App::new(t, arg).into())
        }
        r => Err(UnexpectedRule::new(r, "Application").into()),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        s => Err(UnknownKeyword::new(s).into()),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Lambda::new(var, ty, body).into())
}

fn pair_to_tylambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Type Variable", "Kind Annot", "Type Abstraction Body"],
    )?;
    let var = inner.remove(0).as_str().trim();
    let knd_rule = inner.remove(0);
    let knd = pair_to_kind(knd_rule)?;

    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(TyLambda::new(var, knd, body).into())
}

fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp::new(t, ty).into())
}
