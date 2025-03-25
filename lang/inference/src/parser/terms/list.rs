use super::{pair_to_n_inner, pair_to_prim_term, pair_to_term, Error, Rule};
use crate::syntax::{Cons, Head, IsNil, Tail};
use pest::iterators::Pair;

pub fn pair_to_cons(p: Pair<'_, Rule>) -> Result<Cons, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Cons Keyword",
            "Const First Argument",
            "Cons Second Argument",
        ],
    )?;
    inner.remove(0);
    let fst_rule = inner.remove(0);
    let fst_term = pair_to_term(fst_rule)?;
    let rst_rule = inner.remove(0);
    let rst_term = pair_to_term(rst_rule)?;
    Ok(Cons {
        fst: Box::new(fst_term),
        rst: Box::new(rst_term),
    })
}

pub fn pair_to_isnil(p: Pair<'_, Rule>) -> Result<IsNil, Error> {
    let mut inner = pair_to_n_inner(p, vec!["IsNil Keyword", "IsNil Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(IsNil {
        list: Box::new(arg),
    })
}

pub fn pair_to_head(p: Pair<'_, Rule>) -> Result<Head, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Head Keyword", "Head Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Head {
        list: Box::new(arg),
    })
}

pub fn pair_to_tail(p: Pair<'_, Rule>) -> Result<Tail, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Tail Keyword", "Tail Argument"])?;
    inner.remove(0);
    let arg_rule = inner.remove(0);
    let arg_inner = pair_to_n_inner(arg_rule, vec!["Non Left-Recursive Term"])?.remove(0);
    let arg = pair_to_prim_term(arg_inner)?;
    Ok(Tail {
        list: Box::new(arg),
    })
}
