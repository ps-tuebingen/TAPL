use super::{pair_to_n_inner, pair_to_prim_term, pair_to_term, Error, Rule};
use crate::terms::If;
use pest::iterators::Pair;

pub fn pair_to_if(p: Pair<'_, Rule>) -> Result<If, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "If Keyword",
            "If Condition",
            "Then Branch",
            "Else Keyword",
            "Else Branch",
        ],
    )?;
    inner.remove(0);
    let cond_rule = inner.remove(0);
    let cond = pair_to_prim_term(cond_rule)?;
    let then_rule = inner.remove(0);
    let thent = pair_to_term(then_rule)?;
    inner.remove(0);
    let else_rule = inner.remove(0);
    let elset = pair_to_term(else_rule)?;
    Ok(If {
        ifc: Box::new(cond),
        thenc: Box::new(thent),
        elsec: Box::new(elset),
    })
}
