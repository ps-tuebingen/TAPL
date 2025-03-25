use super::{pair_to_n_inner, pair_to_prim_term, pair_to_term, Error, Rule};
use crate::{
    syntax::{SomeCase, Something, Term},
    Var,
};
use pest::iterators::Pair;

pub fn pair_to_something(p: Pair<'_, Rule>) -> Result<Something, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Keyword Something", "Something Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Something {
        term: Box::new(arg),
    })
}

pub fn pair_to_somecase(p: Pair<'_, Rule>) -> Result<SomeCase, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Case Keyword",
            "Case Bound Term",
            "Of Keyword",
            "Case Patterns",
        ],
    )?;
    inner.remove(0);
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    inner.remove(0);
    let pattern_rule = inner.remove(0);
    pair_to_somepatterns(pattern_rule, bound_term)
}

fn pair_to_somepatterns(p: Pair<'_, Rule>, t: Term) -> Result<SomeCase, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Something/Nothing Pattern", "Something/Nothing Pattern"],
    )?;
    let pt_fst = inner.remove(0);
    let pt_snd = inner.remove(0);
    match (pt_fst.as_rule(), pt_snd.as_rule()) {
        (Rule::nothing_pattern, Rule::something_pattern) => {
            let nothing_rhs = pair_to_nothing_pattern(pt_fst)?;
            let (some_var, some_rhs) = pair_to_something_pattern(pt_snd)?;
            Ok(SomeCase {
                bound_term: Box::new(t),
                none_rhs: Box::new(nothing_rhs),
                some_var,
                some_rhs: Box::new(some_rhs),
            })
        }
        (Rule::something_pattern, Rule::nothing_pattern) => {
            let nothing_rhs = pair_to_nothing_pattern(pt_snd)?;
            let (some_var, some_rhs) = pair_to_something_pattern(pt_fst)?;
            Ok(SomeCase {
                bound_term: Box::new(t),
                none_rhs: Box::new(nothing_rhs),
                some_var,
                some_rhs: Box::new(some_rhs),
            })
        }
        (r, _) => Err(Error::unexpected(r, "Something Patterns")),
    }
}

fn pair_to_nothing_pattern(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Nothing Keyword", "Nothing Right-Hand SIde"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(term)
}

fn pair_to_something_pattern(p: Pair<'_, Rule>) -> Result<(Var, Term), Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Something Keyword",
            "Something Bound Var",
            "Something Right-Hand Side",
        ],
    )?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok((var, term))
}
