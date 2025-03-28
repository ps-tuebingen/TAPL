use super::{pair_to_kind, pair_to_n_inner, pair_to_type, Error, Rule};
use crate::syntax::Term;
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
            let leftrec_inner = pair_to_n_inner(left_rec, vec!["Left Recursive Term"])?.remove(0);
            pair_to_leftrec(leftrec_inner, prim_term)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
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
                .map_err(|_| Error::UnknownKw(p.as_str().to_owned()))?;
            Ok(Term::Const(num))
        }
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        r => Err(Error::unexpected(r, "Non Left-Recusrive Term")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::tyapp => pair_to_tyapp(p, t),
        Rule::term => {
            let arg = pair_to_term(p)?;
            Ok(Term::App {
                fun: Box::new(t),
                arg: Box::new(arg),
            })
        }
        r => Err(Error::unexpected(r, "Application")),
    }
}

fn str_to_term(s: &str) -> Result<Term, Error> {
    match s.to_lowercase().trim() {
        "unit" => Ok(Term::Unit),
        "true" => Ok(Term::True),
        "false" => Ok(Term::False),
        s => Err(Error::UnknownKw(s.to_owned())),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Body"])?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Term::Lambda {
        var,
        annot: ty,
        body: Box::new(body),
    })
}

fn pair_to_tylambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Type Variable", "Kind Annot", "Type Abstraction Body"],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let knd_rule = inner.remove(0);
    let knd = pair_to_kind(knd_rule)?;

    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(Term::TyLambda {
        var,
        kind: knd,
        body: Box::new(body),
    })
}

fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Term::TyApp {
        fun: Box::new(t),
        arg: ty,
    })
}
