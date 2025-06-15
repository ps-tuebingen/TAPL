use super::{pair_to_n_inner, pair_to_term, Error, Rule, Term};
use pest::iterators::Pair;
use syntax::terms::If;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["If Condition", "Then Term", "Else Term"])?;

    let ift_rule = inner.remove(0);
    let ift = pair_to_term(ift_rule)?;
    let thent_rule = inner.remove(0);
    let thent = pair_to_term(thent_rule)?;
    let elset_rule = inner.remove(0);
    let elset = pair_to_term(elset_rule)?;
    Ok(If::new(ift, thent, elset))
}
