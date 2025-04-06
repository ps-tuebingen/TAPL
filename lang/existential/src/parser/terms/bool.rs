use super::{pair_to_n_inner, pair_to_primterm, pair_to_term, Error, Rule};
use crate::terms::If;
use pest::iterators::Pair;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "If Keyword",
            "If Condition",
            "If Then Term",
            "Else Keyword",
            "If Else Term",
        ],
    )?;
    inner.remove(0);
    let cond_rule = inner.remove(0);
    let cond_term = pair_to_primterm(cond_rule)?;
    let then_rule = inner.remove(0);
    let then_term = pair_to_term(then_rule)?;
    inner.remove(0);
    let else_rule = inner.remove(0);
    let else_term = pair_to_term(else_rule)?;
    Ok(If {
        ifc: Box::new(cond_term),
        thenc: Box::new(then_term),
        elsec: Box::new(else_term),
    })
}
