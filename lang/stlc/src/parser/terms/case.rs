use super::pair_to_term;
use crate::{
    parser::{errors::Error, next_rule, Rule},
    syntax::Term,
};
use pest::iterators::Pair;

pub fn pair_to_case(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let mut inner = p.into_inner();
    let bound_term_pair = inner
        .next()
        .ok_or(Error::MissingInput("Case Bound Term".to_owned()))?;
    let bound_term_rule = next_rule(bound_term_pair, Rule::term)?;
    let bound_term = pair_to_term(bound_term_rule)?;
    let pt_fst = inner
        .next()
        .ok_or(Error::MissingInput("Patterns".to_owned()))?;
    let mut pt_inner = pt_fst.into_inner();
    let bnd_fst = pt_inner
        .next()
        .ok_or(Error::MissingInput("Pattern Binding".to_owned()))?;
    let rhs_pair = pt_inner
        .next()
        .ok_or(Error::MissingInput("Pattern Rhs".to_owned()))?;
    let rhs_rule = next_rule(rhs_pair, Rule::term)?;
    let rhs_term = pair_to_term(rhs_rule)?;
    if let Some(n) = pt_inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    let mut fst_bnd_inner = bnd_fst.into_inner();
    let fst_bnd_pair = fst_bnd_inner
        .next()
        .ok_or(Error::MissingInput("Pattern Binding".to_owned()))?;
    if let Some(n) = fst_bnd_inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    match fst_bnd_pair.as_rule() {
        Rule::inl_bnd => {
            println!("{fst_bnd_pair:?}");
            todo!()
        }
        Rule::inr_bnd => {
            println!("{fst_bnd_pair:?}");
            todo!()
        }
        Rule::variant_bnd => {
            println!("{fst_bnd_pair:?}");
            todo!()
        }
        Rule::some_bnd => {
            println!("{fst_bnd_pair:?}");
            todo!()
        }
        Rule::none_bnd => {
            println!("{fst_bnd_pair:?}");
            todo!()
        }
        r => {
            return Err(Error::UnexpectedRule {
                found: r,
                expected: Rule::inl_bnd,
            })
        }
    }
}
