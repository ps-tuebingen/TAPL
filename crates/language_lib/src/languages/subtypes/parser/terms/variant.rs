use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, MissingInput, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{variantcase::VariantPattern, Variant, VariantCase};

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Label",
            "Variant Term",
            "As Keyword",
            "Variant Type",
        ],
    )?;
    let label = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    inner.remove(0);
    let ty_rule = pair_to_n_inner(inner.remove(0), vec!["Type"])?.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Variant::new(label, term, ty))
}

pub fn pair_to_variantcase(p: Pair<'_, Rule>) -> Result<VariantCase<Term>, Error> {
    let mut inner = p.into_inner();
    inner.next().ok_or(MissingInput::new("Case Keyword"))?;
    let term_rule = inner
        .next()
        .ok_or(MissingInput::new("Variant Case Bound Term"))?;
    let term = pair_to_term(term_rule)?;
    inner.next().ok_or(MissingInput::new("Of Keyword"))?;
    let mut patterns = vec![];
    for next in inner {
        let pt = pair_to_variant_pattern(next)?;
        patterns.push(pt);
    }

    Ok(VariantCase::new(term, patterns))
}

fn pair_to_variant_pattern(p: Pair<'_, Rule>) -> Result<VariantPattern<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Label",
            "Variant Bound Variable",
            "Pattern Right-Hand Side",
        ],
    )?;
    let label = inner.remove(0).as_str().trim();
    let var = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(VariantPattern::new(label, var, term))
}
