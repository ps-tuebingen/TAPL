use super::pair_to_term;
use crate::{
    parser::{errors::Error, pair_to_n_inner, types::pair_to_type, Rule},
    syntax::{Cons, ListCase, Nil, Term, Var},
};
use pest::iterators::Pair;

enum ListPattern {
    NilPattern {
        rhs: Term,
    },
    ConsPattern {
        fst_var: Var,
        rst_var: Var,
        rhs: Term,
    },
}

pub fn pair_to_nil(p: Pair<'_, Rule>) -> Result<Nil, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Nil Keyword", "Nil Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_pair)?;
    Ok(Nil { ty })
}

pub fn pair_to_cons(p: Pair<'_, Rule>) -> Result<Cons, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Cons Keyword",
            "First Cons Argument",
            "Second Cons Argument",
        ],
    )?;
    inner.remove(0);
    let fst_rule = inner.remove(0);
    let fst = pair_to_term(fst_rule)?;
    let rst_rule = inner.remove(0);
    let rst = pair_to_term(rst_rule)?;
    Ok(Cons {
        fst: Box::new(fst),
        rst: Box::new(rst),
    })
}

pub fn pair_to_listcase(p: Pair<'_, Rule>) -> Result<ListCase, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Case Keyword",
            "Case Bound Term",
            "Case Type",
            "Of Keyword",
            "First List Pattern (Nil or Cons)",
            "Second List Pattern (Nil,Cons)",
        ],
    )?;
    inner.remove(0);
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
    let ty_rule = inner.remove(0);
    let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let list_ty = pair_to_type(ty_pair)?;
    inner.remove(0);
    let pt_fst_pair = inner.remove(0);
    let pt_fst = pair_to_list_pattern(pt_fst_pair)?;
    let pt_rst_pair = inner.remove(0);
    let pt_rst = pair_to_list_pattern(pt_rst_pair)?;

    match (pt_fst, pt_rst) {
        (
            ListPattern::NilPattern { rhs: nil_rhs },
            ListPattern::ConsPattern {
                fst_var,
                rst_var,
                rhs: cons_rhs,
            },
        ) => Ok(ListCase {
            bound_term: Box::new(bound_term),
            list_ty,
            nil_rhs: Box::new(nil_rhs),
            cons_fst: fst_var,
            cons_rst: rst_var,
            cons_rhs: Box::new(cons_rhs),
        }),
        (
            ListPattern::ConsPattern {
                fst_var,
                rst_var,
                rhs: cons_rhs,
            },
            ListPattern::NilPattern { rhs: nil_rhs },
        ) => Ok(ListCase {
            bound_term: Box::new(bound_term),
            list_ty,
            nil_rhs: Box::new(nil_rhs),
            cons_fst: fst_var,
            cons_rst: rst_var,
            cons_rhs: Box::new(cons_rhs),
        }),
        (ListPattern::NilPattern { .. }, ListPattern::NilPattern { .. }) => {
            Err(Error::MissingCtor("Cons".to_owned()))
        }
        (ListPattern::ConsPattern { .. }, ListPattern::ConsPattern { .. }) => {
            Err(Error::MissingCtor("Nil".to_owned()))
        }
    }
}

fn pair_to_list_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    match p.as_rule() {
        Rule::nil_pt => pair_to_nil_pattern(p),
        Rule::cons_pt => pair_to_cons_pattern(p),
        r => Err(Error::unexpected(r, "Nil or Cons Pattern")),
    }
}

fn pair_to_nil_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Nil Keyword", "Nil Right-Hand Side"])?;
    inner.remove(0);
    let rhs_pair = inner.remove(0);
    let rhs = pair_to_term(rhs_pair)?;
    Ok(ListPattern::NilPattern { rhs })
}

fn pair_to_cons_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Cons Keyword",
            "Cons First Bound Variable",
            "Cons Second Bound Variable",
            "Cons Rhs",
        ],
    )?;
    inner.remove(0);
    let fst_var = inner.remove(0).as_str().trim().to_owned();
    let rst_var = inner.remove(0).as_str().trim().to_owned();
    let rhs_pair = inner.remove(0);
    let rhs = pair_to_term(rhs_pair)?;

    Ok(ListPattern::ConsPattern {
        fst_var,
        rst_var,
        rhs,
    })
}
