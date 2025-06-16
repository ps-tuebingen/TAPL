use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{LambdaSub, TyApp};

pub fn pair_to_tylambda(p: Pair<'_, Rule>) -> Result<LambdaSub<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Type Variable", "Type Abstraction Body"])?;
    let var = inner.remove(0).as_str().trim();
    let body_rule = inner.remove(0);
    let body = pair_to_term(body_rule)?;
    Ok(LambdaSub::new_unbounded(var, body))
}

pub fn pair_to_tyapp(p: Pair<'_, Rule>, t: Term) -> Result<TyApp<Term, Type>, Error> {
    let ty_rule = pair_to_n_inner(p, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(TyApp::new(t, ty))
}
