use super::{pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::{App, Lambda, Term, TyApp, TyLambda};
use pest::iterators::Pair;

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
        Rule::lambda_term => pair_to_lambda(p).map(|lam| lam.into()),
        Rule::tylambda_term => pair_to_tylambda(p).map(|tylam| tylam.into()),
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recursive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::tyapp => {
            let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
            let ty = pair_to_type(ty_rule)?;
            Ok(TyApp {
                term: Box::new(t),
                ty,
            }
            .into())
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
        r => Err(Error::unexpected(r, "Left Recursive Term")),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Lambda, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let ty_rule = inner.remove(0);
    let annot = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Lambda {
        var,
        annot,
        body: Box::new(body),
    })
}

fn pair_to_tylambda(p: Pair<'_, Rule>) -> Result<TyLambda, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(TyLambda {
        var,
        term: Box::new(body),
    })
}
