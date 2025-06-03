use super::{pair_to_n_inner, pair_to_prim_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::{Pred, Succ};

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Succ Keyword", "Succ Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Succ::new(term))
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pred Keyword", "Pred Argument"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Pred::new(term))
}
