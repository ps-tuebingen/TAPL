use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::terms::{Variant, VariantCase, VariantPattern};
use pest::iterators::Pair;

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Variable",
            "Variant Term",
            "As Keyword",
            "Variant Type",
        ],
    )?;
    let var = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    inner.remove(0);
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Variant {
        label: var,
        term: Box::new(term),
        annot: ty,
    })
}

pub fn pair_to_variantcase(p: Pair<'_, Rule>) -> Result<VariantCase, Error> {
    let mut inner = p.into_inner();
    inner
        .next()
        .ok_or(Error::MissingInput("Case Keyword".to_owned()))?;
    let bound_rule = inner
        .next()
        .ok_or(Error::MissingInput("Case Bound Term".to_owned()))?;
    let bound_term = pair_to_term(bound_rule)?;
    inner
        .next()
        .ok_or(Error::MissingInput("Of Keyword".to_owned()))?;
    let mut patterns = vec![];
    for pattern_rule in inner {
        patterns.push(pair_to_variantpattern(pattern_rule)?);
    }
    Ok(VariantCase {
        bound_term: Box::new(bound_term),
        patterns,
    })
}

fn pair_to_variantpattern(p: Pair<'_, Rule>) -> Result<VariantPattern, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Pattern Label",
            "Variant Pattern Var",
            "Variant Pattern Right-Hand Side",
        ],
    )?;
    let label = inner.remove(0).as_str().trim().to_owned();
    let var = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(VariantPattern {
        label,
        bound_var: var,
        rhs: Box::new(term),
    })
}
