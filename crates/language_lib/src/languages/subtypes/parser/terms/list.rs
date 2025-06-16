use super::{
    pair_to_n_inner, pair_to_term, pair_to_type, Error, MissingInput, Rule, Term, Type,
    UnexpectedRule,
};
use pest::iterators::Pair;
use syntax::{
    terms::{Cons, ListCase, Nil},
    Var,
};

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

pub fn pair_to_nil(p: Pair<'_, Rule>) -> Result<Nil<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Nil Type"])?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Nil::new(ty))
}

pub fn pair_to_cons(p: Pair<'_, Rule>) -> Result<Cons<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Cons Type", "First Cons Argument", "Second Cons Argument"],
    )?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;

    let fst_rule = inner.remove(0);
    let fst = pair_to_term(fst_rule)?;
    let rst_rule = inner.remove(0);
    let rst = pair_to_term(rst_rule)?;
    Ok(Cons::new(fst, rst, ty))
}

pub fn pair_to_listcase(p: Pair<'_, Rule>) -> Result<ListCase<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Case Bound Term",
            "First List Pattern (Nil or Cons)",
            "Second List Pattern (Nil,Cons)",
        ],
    )?;
    let bound_rule = inner.remove(0);
    let bound_term = pair_to_term(bound_rule)?;
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
        ) => Ok(ListCase::new(
            bound_term, nil_rhs, &fst_var, &rst_var, cons_rhs,
        )),
        (
            ListPattern::ConsPattern {
                fst_var,
                rst_var,
                rhs: cons_rhs,
            },
            ListPattern::NilPattern { rhs: nil_rhs },
        ) => Ok(ListCase::new(
            bound_term, nil_rhs, &fst_var, &rst_var, cons_rhs,
        )),
        (ListPattern::NilPattern { .. }, ListPattern::NilPattern { .. }) => {
            Err(MissingInput::new("Cons Pattern").into())
        }
        (ListPattern::ConsPattern { .. }, ListPattern::ConsPattern { .. }) => {
            Err(MissingInput::new("Nil Pattern").into())
        }
    }
}

fn pair_to_list_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    match p.as_rule() {
        Rule::nil_pattern => pair_to_nil_pattern(p),
        Rule::cons_pattern => pair_to_cons_pattern(p),
        r => Err(UnexpectedRule::new(r, "Nil or Cons Pattern").into()),
    }
}

fn pair_to_nil_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Nil Right-Hand Side"])?;
    let rhs_pair = inner.remove(0);
    let rhs = pair_to_term(rhs_pair)?;
    Ok(ListPattern::NilPattern { rhs })
}

fn pair_to_cons_pattern(p: Pair<'_, Rule>) -> Result<ListPattern, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Cons First Bound Variable",
            "Cons Second Bound Variable",
            "Cons Rhs",
        ],
    )?;
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
