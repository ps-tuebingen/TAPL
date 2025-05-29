use super::{pair_to_n_inner, pair_to_type, to_parse_err, Rule, Term, Type};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use syntax::{
    kinds::Kind,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
};

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();

    let prim_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Non Left-Recursive Term".to_owned(),
    )))?;
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
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            n.as_rule()
        ))));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::tylambda_term => pair_to_tylambda(p).map(|tylam| tylam.into()),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
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
        Rule::tyapp => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            let ty = pair_to_type(ty_rule)?;
            Ok(TyApp::new(t, ty).into())
        }
        Rule::paren_tyapp => {
            let ty_app_rule = pair_to_n_inner(p, vec!["Type Application"])?.remove(0);
            pair_to_leftrec(ty_app_rule, t)
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
            expected: "Left Recursive Term".to_owned(),
        })),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim();
    let ty_rule = inner.remove(0);
    let annot = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Lambda::new(var, annot, body))
}

fn pair_to_tylambda(p: Pair<'_, Rule>) -> Result<TyLambda<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
    let var = inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(TyLambda::new(var, Kind::Star, body))
}
