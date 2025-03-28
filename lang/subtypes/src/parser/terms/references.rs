use super::{pair_to_prim_term, pair_to_term};
use crate::{
    parser::{errors::Error, pair_to_n_inner, Rule},
    syntax::{Assign, Deref, Ref, Term},
};
use pest::iterators::Pair;

pub fn pair_to_ref(p: Pair<'_, Rule>) -> Result<Ref, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Ref Keyword", "Ref Term"])?;
    inner.remove(0);
    let term_rule = inner.remove(0);
    let term = pair_to_prim_term(term_rule)?;
    Ok(Ref {
        term: Box::new(term),
    })
}

pub fn pair_to_deref(p: Pair<'_, Rule>) -> Result<Deref, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Deref {
        term: Box::new(term),
    })
}

pub fn pair_to_assign(p: Pair<'_, Rule>, t: Term) -> Result<Assign, Error> {
    let term_rule = pair_to_n_inner(p, vec!["Assign Right-hand Side"])?.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Assign {
        to: Box::new(t),
        content: Box::new(term),
    })
}
