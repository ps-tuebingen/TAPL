use super::{get_n_inner, next_rule, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::If;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["If Term", "Then Term", "Else Term"])?;

    let if_pair = inner.remove(0);
    let if_rule = next_rule(if_pair, Rule::paren_term)?;
    let if_term = pair_to_term(if_rule)?;

    let then_pair = inner.remove(0);
    let then_term = pair_to_term(then_pair)?;

    let else_pair = inner.remove(0);
    let else_term = pair_to_term(else_pair)?;
    Ok(If::new(if_term, then_term, else_term))
}
