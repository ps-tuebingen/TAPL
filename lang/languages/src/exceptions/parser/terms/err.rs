use super::{pair_to_n_inner, pair_to_term, pair_to_type, Rule, Term};
use common::{
    errors::Error,
    terms::{Exception, Raise, Try, TryWithVal},
};
use pest::iterators::Pair;

pub fn pair_to_try_with(p: Pair<'_, Rule>) -> Result<Try<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Try Keyword", "Try Term", "With Keyword", "With Term"],
    )?;

    inner.remove(0);
    let tryt_rule = inner.remove(0);
    let try_t = pair_to_term(tryt_rule)?;
    inner.remove(0);
    let with_rule = inner.remove(0);
    let witht = pair_to_term(with_rule)?;

    Ok(Try::new(try_t, witht))
}

pub fn pair_to_try_catch(p: Pair<'_, Rule>) -> Result<TryWithVal<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Try Keyword", "Try Term", "Catch Keyword", "Catch Term"],
    )?;

    inner.remove(0);
    let tryt_rule = inner.remove(0);
    let tryt = pair_to_term(tryt_rule)?;
    inner.remove(0);
    let catch_rule = inner.remove(0);
    let catch_term = pair_to_term(catch_rule)?;
    Ok(TryWithVal::new(tryt, catch_term))
}

pub fn pair_to_raise(p: Pair<'_, Rule>) -> Result<Raise<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Raise Keyword",
            "Raise Continuation Type",
            "Raise Term",
            "Raise Exception Type",
        ],
    )?;
    inner.remove(0);
    let cont_ty_rule = inner.remove(0);
    let cont_ty_pair = pair_to_n_inner(cont_ty_rule, vec!["Type"])?.remove(0);
    let cont_ty = pair_to_type(cont_ty_pair)?;
    let catch_rule = inner.remove(0);
    let catch_term = pair_to_term(catch_rule)?;
    let ex_ty_rule = inner.remove(0);
    let ex_ty_pair = pair_to_n_inner(ex_ty_rule, vec!["Type"])?.remove(0);
    let ex_ty = pair_to_type(ex_ty_pair)?;
    Ok(Raise::new(catch_term, cont_ty, ex_ty))
}

pub fn pair_to_err(p: Pair<'_, Rule>) -> Result<Exception<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Error Keyword", "Error Type"])?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_pair)?;
    Ok(Exception::new(ty))
}
