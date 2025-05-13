use super::{pair_to_n_inner, to_parse_err, types::pair_to_type, Rule, Term};
use common::{
    errors::{Error, ErrorKind},
    terms::{
        App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Num, Pred, Ref, Succ, True, Unit,
        Variable,
    },
    types::Unit as UnitTy,
};
use pest::iterators::Pair;

pub fn pair_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    if p.as_rule() != Rule::term {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{:?}", p.as_rule()),
            expected: "Term".to_owned(),
        }));
    }
    let mut inner = p.into_inner();
    let prim_pair = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Term (non-left recursive)".to_owned(),
    )))?;
    let prim_term = pair_to_prim_term(prim_pair)?;
    let term = match inner.next() {
        Some(p) => pair_to_leftrec(p, prim_term)?,
        None => prim_term,
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
    if p.as_rule() != Rule::prim_term {
        return Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{:?}", p.as_rule()),
            expected: "Term (non-left recursive)".to_owned(),
        }));
    }
    let inner_rule = pair_to_n_inner(p, vec!["Term (non-left recursive)"])?.remove(0);
    prim_rule_to_term(inner_rule)
}

fn prim_rule_to_term(p: Pair<'_, Rule>) -> Result<Term, Error> {
    match p.as_rule() {
        Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
        Rule::number => {
            let num = p
                .as_str()
                .trim()
                .parse::<i64>()
                .map_err(|_| to_parse_err(ErrorKind::UnknownKeyword(p.as_str().to_owned())))?;
            Ok(Num::new(num).into())
        }
        Rule::paren_term => {
            let term_rule = pair_to_n_inner(p, vec!["Term"])?.remove(0);
            pair_to_term(term_rule)
        }
        Rule::r#const => const_to_term(p.as_str()),
        Rule::lambda_term => pair_to_lambda(p),
        Rule::succ_term => pair_to_succ(p),
        Rule::pred_term => pair_to_pred(p),
        Rule::iszero_term => pair_to_isz(p),
        Rule::ref_term => pair_to_ref(p),
        Rule::deref_term => pair_to_deref(p),
        Rule::let_term => pair_to_let(p),
        Rule::if_term => pair_to_if(p),
        Rule::fix_term => {
            let mut fix_inner = pair_to_n_inner(p, vec!["Fix Keyword", "Fix Term"])?;
            fix_inner.remove(0);
            let paren_rule = fix_inner.remove(0);
            let fix_term = prim_rule_to_term(paren_rule)?;
            Ok(Fix::new(fix_term).into())
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Term (non-left recursive)".to_owned(),
        })),
    }
}

fn pair_to_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
    let inner_rule = pair_to_n_inner(p, vec!["Left Recursive Term"])?.remove(0);
    match inner_rule.as_rule() {
        Rule::assign_term => {
            let term_rule = pair_to_n_inner(inner_rule, vec!["Assign Right hand side"])?.remove(0);
            let rhs = pair_to_term(term_rule)?;
            Ok(Assign::new(t, rhs).into())
        }
        Rule::sequence => {
            let term_rule = pair_to_n_inner(inner_rule, vec!["Sequence Second Term"])?.remove(0);
            let term = pair_to_term(term_rule)?;
            Ok(App::new(Lambda::new("_", UnitTy::new(), term), t).into())
        }
        Rule::term => {
            let arg = pair_to_term(inner_rule)?;
            Ok(App::new(t, arg).into())
        }
        r => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{r:?}"),
            expected: "Assign or Application".to_owned(),
        })),
    }
}

fn const_to_term(c: &str) -> Result<Term, Error> {
    match c.to_lowercase().trim() {
        "unit" => Ok(Unit::new().into()),
        "true" => Ok(True::new().into()),
        "false" => Ok(False::new().into()),
        s => Err(to_parse_err(ErrorKind::UnknownKeyword(s.to_owned()))),
    }
}

fn pair_to_lambda(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner_rules =
        pair_to_n_inner(p, vec!["Lambda Variable", "Lambda Annot", "Lambda Term"])?;
    let var = inner_rules.remove(0).as_str().trim();

    let ty_rule = inner_rules.remove(0);
    let annot = pair_to_type(ty_rule)?;

    let term_rule = inner_rules.remove(0);
    let term = prim_rule_to_term(term_rule)?;

    Ok(Lambda::new(var, annot, term).into())
}

fn pair_to_ref(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner_rules = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Term"])?;
    let _ = inner_rules.remove(0);
    let term_rule = inner_rules.remove(0);
    let term = prim_rule_to_term(term_rule)?;
    Ok(Ref::new(term).into())
}

fn pair_to_deref(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Deref::new(term).into())
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
    let var = inner.remove(0).as_str().trim();

    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    inner.remove(0);
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;

    Ok(Let::new(var, bound_term, in_term).into())
}

fn pair_to_if(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "If Keyword",
            "If Condition",
            "then Term",
            "Else Keyword",
            "else Term",
        ],
    )?;
    inner.remove(0);
    let cond_pair = inner.remove(0);
    let cond_term = pair_to_term(cond_pair)?;
    let then_pair = inner.remove(0);
    let then_term = pair_to_term(then_pair)?;
    inner.remove(0);
    let else_pair = inner.remove(0);
    let else_term = pair_to_term(else_pair)?;
    Ok(If::new(cond_term, then_term, else_term).into())
}

fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Keyword Succ", "Succ Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = prim_rule_to_term(term_rule)?;
    Ok(Succ::new(term).into())
}

fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Keyword Pred", "Pred Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = prim_rule_to_term(term_rule)?;
    Ok(Pred::new(term).into())
}
fn pair_to_isz(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Keyword IsZero", "IsZero Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = prim_rule_to_term(term_rule)?;
    Ok(IsZero::new(term).into())
}
