use super::{get_n_inner, next_rule, pair_to_term, pair_to_type, Rule, Term, Type};
use common::{
    errors::{Error, ErrorKind},
    parse::to_parse_err,
};
use pest::iterators::Pair;
use syntax::terms::{Cons, Head, IsNil, Nil, Tail};

pub fn pair_to_nil(p: Pair<'_, Rule>) -> Result<Nil<Term, Type>, Error> {
    let ty_pair = get_n_inner(p, vec!["Nil Type"])?.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    Ok(Nil::new(ty))
}

pub fn pair_to_cons(p: Pair<'_, Rule>) -> Result<Cons<Term, Type>, Error> {
    let mut inner = get_n_inner(
        p,
        vec!["Cons Type", "First Const Argument", "Second Cons Argument"],
    )?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    let fst_pair = inner.remove(0);
    let fst = pair_to_term(fst_pair)?;

    let snd_pair = inner.remove(0);
    let snd = pair_to_term(snd_pair)?;

    Ok(Cons::new(fst, snd, ty))
}

pub fn pair_to_head(p: Pair<'_, Rule>) -> Result<Head<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["Head Type", "Head Argument"])?;

    let ty_rule = inner.remove(0);
    let ty_pair = next_rule(ty_rule, Rule::r#type)?;
    let ty = pair_to_type(ty_pair)?;

    let term_pair = inner.remove(0);
    let term = pair_to_term(term_pair)?;

    Ok(Head::new(term, ty))
}

pub fn pair_to_tail(p: Pair<'_, Rule>) -> Result<Tail<Term, Type>, Error> {
    let mut inner = p.into_inner();
    let ty_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Head Type".to_owned(),
    )))?;
    let ty_pair = next_rule(ty_rule, Rule::r#type)?;
    let ty = pair_to_type(ty_pair)?;

    let term_pair = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Head Argument".to_owned(),
    )))?;
    let term = pair_to_term(term_pair)?;

    if let Some(next) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!(
            "{:?}",
            next.as_rule()
        ))));
    }
    Ok(Tail::new(term, ty))
}

pub fn pair_to_isnil(p: Pair<'_, Rule>) -> Result<IsNil<Term, Type>, Error> {
    let mut inner = get_n_inner(p, vec!["IsNil Type", "IsNil Argument"])?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let ty = pair_to_type(ty_rule)?;

    let term_pair = inner.remove(0);
    let term = pair_to_term(term_pair)?;

    Ok(IsNil::new(term, ty))
}
