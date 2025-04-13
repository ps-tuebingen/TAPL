use super::{pair_to_term, to_parse_err};
use crate::{
    parser::{pair_to_n_inner, types::pair_to_type, Rule},
    syntax::{variant::VariantPattern, Variant, VariantCase},
};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Variant Label", "Variant Term"])?;
    let label = inner.remove(0).as_str().trim().to_owned();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(Variant {
        label,
        term: Box::new(term),
    })
}

pub fn pair_to_variantcase(p: Pair<'_, Rule>) -> Result<VariantCase, Error> {
    let mut inner = p.into_inner();
    inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Case Keyword".to_owned(),
    )))?;
    let term_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Variant Case Bound Term".to_owned(),
    )))?;
    let term = pair_to_term(term_rule)?;
    inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Of Keyword".to_owned(),
    )))?;

    let mut patterns = vec![];
    for next in inner {
        let pt = pair_to_variant_pattern(next)?;
        patterns.push(pt);
    }

    Ok(VariantCase {
        bound_term: Box::new(term),
        patterns,
    })
}

fn pair_to_variant_pattern(p: Pair<'_, Rule>) -> Result<VariantPattern, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Label",
            "Variant Bound Variable",
            "Variant Type",
            "Pattern Right-Hand Side",
        ],
    )?;
    let label = inner.remove(0).as_str().trim().to_owned();
    let var = inner.remove(0).as_str().trim().to_owned();
    let ty_rule = inner.remove(0);
    let ty_pair = pair_to_n_inner(ty_rule, vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_pair)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(VariantPattern {
        label,
        bound_var: var,
        var_ty: ty,
        rhs: Box::new(term),
    })
}
