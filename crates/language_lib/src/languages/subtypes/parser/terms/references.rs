use super::{pair_to_n_inner, pair_to_prim_term, pair_to_term, Rule, Term};
use common::errors::Error;
use pest::iterators::Pair;
use syntax::terms::{Assign, Deref, Ref};

pub fn pair_to_ref(p: Pair<'_, Rule>) -> Result<Ref<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Term"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Ref::new(term))
}

pub fn pair_to_deref(p: Pair<'_, Rule>) -> Result<Deref<Term>, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Deref::new(term))
}

pub fn pair_to_assign(p: Pair<'_, Rule>, t: Term) -> Result<Assign<Term>, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Assign Right-hand Side"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Assign::new(t, term))
}
