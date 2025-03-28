use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::terms::{Fold, Unfold};
use pest::iterators::Pair;

pub fn pair_to_fold(p: Pair<'_, Rule>) -> Result<Fold, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fold Keyword", "Fold Type", "fold Term"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Fold {
        ty,
        term: Box::new(term),
    })
}

pub fn pair_to_unfold(p: Pair<'_, Rule>) -> Result<Unfold, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Unfold Keyword", "Unfold Type", "Unfold Term"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Unfold {
        ty,
        term: Box::new(term),
    })
}
