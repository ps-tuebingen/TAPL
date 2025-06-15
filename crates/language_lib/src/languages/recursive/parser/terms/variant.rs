use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, MissingInput, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{variantcase::VariantPattern, Variant, VariantCase};

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Variant Variable", "Variant Term", "Variant Type"])?;
    let var = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    let ty_rule = inner.remove(0);
    let ty = pair_to_type(ty_rule)?;
    Ok(Variant::new(var, term, ty))
}

pub fn pair_to_variantcase(p: Pair<'_, Rule>) -> Result<VariantCase<Term>, Error> {
    let mut inner = p.into_inner();
    let bound_rule = inner.next().ok_or(MissingInput::new("Case Bound Term"))?;
    let bound_term = pair_to_term(bound_rule)?;
    let mut patterns = vec![];
    for pattern_rule in inner {
        patterns.push(pair_to_variantpattern(pattern_rule)?);
    }
    Ok(VariantCase::new(bound_term, patterns))
}

fn pair_to_variantpattern(p: Pair<'_, Rule>) -> Result<VariantPattern<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Variant Pattern Label",
            "Variant Pattern Var",
            "Variant Pattern Right-Hand Side",
        ],
    )?;
    let label = inner.remove(0).as_str().trim();
    let var = inner.remove(0).as_str().trim();
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    Ok(VariantPattern::new(label, var, term))
}
