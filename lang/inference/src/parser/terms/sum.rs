use super::{pair_to_n_inner, pair_to_prim_term, pair_to_term, Error, Rule};
use crate::{
    syntax::{Left, Right, SumCase, Term},
    Var,
};
use pest::iterators::Pair;

pub fn pair_to_left(p: Pair<'_, Rule>) -> Result<Left, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Inl Keyword", "Inl Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Left {
        left_term: Box::new(arg),
    })
}

pub fn pair_to_right(p: Pair<'_, Rule>) -> Result<Right, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Inr Keyword", "Inr Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Right {
        right_term: Box::new(arg),
    })
}

pub fn pair_to_sumcase(p: Pair<'_, Rule>) -> Result<SumCase, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Case Keyword",
            "Case Bound Term",
            "Of Keyword",
            "Sum Case Patterns",
        ],
    )?;
    inner.remove(0);
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    let pt_rule = inner.remove(0);
    pair_to_sumcase_patterns(pt_rule, bound_term)
}

fn pair_to_sumcase_patterns(p: Pair<'_, Rule>, t: Term) -> Result<SumCase, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Left/Right Pattern", "Left/Right Pattern"])?;
    let pt_fst = inner.remove(0);
    let pt_snd = inner.remove(0);
    match (pt_fst.as_rule(), pt_snd.as_rule()) {
        (Rule::left_pattern, Rule::right_pattern) => {
            let (left_var, left_rhs) = pair_to_sum_pattern(pt_fst)?;
            let (right_var, right_rhs) = pair_to_sum_pattern(pt_snd)?;
            Ok(SumCase {
                bound_term: Box::new(t),
                right_var,
                right_term: Box::new(right_rhs),
                left_var,
                left_term: Box::new(left_rhs),
            })
        }
        (Rule::right_pattern, Rule::left_pattern) => {
            let (left_var, left_rhs) = pair_to_sum_pattern(pt_snd)?;
            let (right_var, right_rhs) = pair_to_sum_pattern(pt_fst)?;
            Ok(SumCase {
                bound_term: Box::new(t),
                right_var,
                right_term: Box::new(right_rhs),
                left_var,
                left_term: Box::new(left_rhs),
            })
        }
        (r, _) => Err(Error::unexpected(r, "Sum Case Patterns")),
    }
}

fn pair_to_sum_pattern(p: Pair<'_, Rule>) -> Result<(Var, Term), Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Sum Constructor",
            "Pattern Bound Variable",
            "Pattern Right-Hand Side",
        ],
    )?;
    inner.remove(0);
    let var = inner.remove(0).as_str().trim().to_owned();
    let rhs_rule = inner.remove(0);
    let rhs = pair_to_term(rhs_rule)?;
    Ok((var, rhs))
}
