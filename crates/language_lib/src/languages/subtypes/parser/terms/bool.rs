use super::{pair_to_n_inner, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::If;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;
    let cond_rule = inner.remove(0);
    let cond_term = pair_to_term(cond_rule)?;
    let then_rule = inner.remove(0);
    let then_term = pair_to_term(then_rule)?;
    let else_rule = inner.remove(0);
    let else_term = pair_to_term(else_rule)?;

    Ok(If::new(cond_term, then_term, else_term))
}
