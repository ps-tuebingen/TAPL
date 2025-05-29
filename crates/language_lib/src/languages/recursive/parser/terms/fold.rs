use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{Fold, Unfold};

pub fn pair_to_fold(p: Pair<'_, Rule>) -> Result<Fold<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Fold Keyword", "Fold Type", "fold Term"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Fold::new(term, ty))
}

pub fn pair_to_unfold(p: Pair<'_, Rule>) -> Result<Unfold<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Unfold Keyword", "Unfold Type", "Unfold Term"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Unfold::new(ty, term))
}
