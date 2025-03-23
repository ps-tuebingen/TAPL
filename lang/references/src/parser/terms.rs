use super::{errors::Error, pair_to_n_inner, types::pair_to_type, Rule};
use crate::{terms::Term, types::Type};
use pest::iterators::Pair;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    if p.as_rule() != Rule::term {
        return Err(Error::unexpected(p.as_rule(), "Term"));
    }
    let mut inner = p.into_inner();
    let prim_pair = inner
        .next()
        .ok_or(Error::missing("Term (non-left recursive)"))?;
    let prim_term = pair_to_prim_term(prim_pair)?;
    let term = match inner.next() {
        Some(p) => pair_to_leftrec(p, prim_term)?,
        None => prim_term,
    };
    if let Some(n) = inner.next() {
        return Err(Error::remaining(n.as_rule()));
    }
    Ok(term)
}

fn pair_to_prim_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    if p.as_rule() != Rule::prim_term {
        return Err(Error::unexpected(p.as_rule(), "Term (non-left recursive)"));
    }
    let inner_rule = pair_to_n_inner(p, vec!["Term (non-left recursive)"])?.remove(0);
    prim_rule_to_term(inner_rule)
}

fn prim_rule_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Term::Var(p.as_str().trim().to_owned())),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::num(p.as_str()))?;
            Ok(Term::Const(num))
        }
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::r#const => const_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p),
        Rule::ref_term => pair_to_ref(p),
        Rule::deref_term => pair_to_deref(p),
        Rule::let_term => pair_to_let(p),
        r => Err(Error::unexpected(r, "Term (non-left recursive)")),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let inner_rule = pair_to_n_inner(p, vec!["Left Recursive Term"])?.remove(0);
    match inner_rule.as_rule() {
        Rule::assign_term => {
            let term_rule = pair_to_n_inner(inner_rule, vec!["Assign Right hand side"])?.remove(0);
            let rhs = pair_to_term(term_rule)?;
            Ok(Term::Assign {
                to: Box::new(t),
                body: Box::new(rhs),
            })
        }
        Rule::sequence => {
            let term_rule = pair_to_n_inner(inner_rule, vec!["Sequence Second Term"])?.remove(0);
            let term = pair_to_term(term_rule)?;
            Ok(Term::App {
                fun: Box::new(Term::Lambda {
                    var: "_".to_owned(),
                    annot: Type::Unit,
                    body: Box::new(term),
                }),
                arg: Box::new(t),
            })
        }
        Rule::term => {
            let arg = pair_to_term(inner_rule)?;
            Ok(Term::App {
                fun: Box::new(t),
                arg: Box::new(arg),
            })
        }
        r => Err(Error::unexpected(r, "Assign or Application")),
    }
}

fn const_to_term(c: &str) -> Result<Term, Error> {
    match c.to_lowercase().trim() {
        "unit" => Ok(Term::Unit),
        s => Err(Error::kw(s)),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner_rules =
        pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Term"])?;
    let var = inner_rules.remove(0).as_str().trim().to_owned();

    let ty_rule = inner_rules.remove(0);
    let annot = pair_to_type(ty_rule)?;

    let term_rule = inner_rules.remove(0);
    let term = prim_rule_to_term(term_rule)?;

    Ok(Term::Lambda {
        var,
        annot,
        body: Box::new(term),
    })
}

fn pair_to_ref(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner_rules = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Term"])?;
    let _ = inner_rules.remove(0);
    let term_rule = inner_rules.remove(0);
    let term = prim_rule_to_term(term_rule)?;
    Ok(Term::Ref(Box::new(term)))
}

fn pair_to_deref(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Term::Deref(Box::new(term)))
}

fn pair_to_let(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Keyword Let",
            "Let Variable",
            "Let Bound Term",
            "Keyword in",
            "Let In Term",
        ],
    )?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim().to_owned();

    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    inner.remove(0);
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;

    Ok(Term::Let {
        var,
        bound_term: Box::new(bound_term),
        in_term: Box::new(in_term),
    })
}
