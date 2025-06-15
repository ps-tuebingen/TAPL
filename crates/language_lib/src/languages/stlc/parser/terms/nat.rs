use super::{get_n_inner, pair_to_primterm, Error, Rule, Term, UnknownKeyword};
use pest::iterators::Pair;
use syntax::terms::{IsZero, Num, Pred, Succ};

pub fn pair_to_num(p: Pair<'_, Rule>) -> Result<Term, Error> {
    let num_str = p.as_str().trim();
    let num = num_str
        .parse::<i64>()
        .map_err(|_| UnknownKeyword::new(num_str))?;
    Ok(Num::new(num).into())
}

pub fn pair_to_pred(p: Pair<'_, Rule>) -> Result<Pred<Term>, Error> {
    let inner_pair = get_n_inner(p, vec!["Pred Argument"])?.remove(0);
    let inner_term = pair_to_primterm(inner_pair)?;
    Ok(Pred::new(inner_term))
}

pub fn pair_to_succ(p: Pair<'_, Rule>) -> Result<Succ<Term>, Error> {
    let inner_pair = get_n_inner(p, vec!["Succ Argument"])?.remove(0);
    let inner_term = pair_to_primterm(inner_pair)?;
    Ok(Succ::new(inner_term))
}

pub fn pair_to_isz(p: Pair<'_, Rule>) -> Result<IsZero<Term>, Error> {
    let term_pair = get_n_inner(p, vec!["IsZero Argument"])?.remove(0);
    let term = pair_to_primterm(term_pair)?;
    Ok(IsZero::new(term))
}
