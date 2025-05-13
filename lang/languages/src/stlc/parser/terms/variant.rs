use super::{get_n_inner, next_rule, pair_to_term, pair_to_type, Rule, Term};
use common::{errors::Error, terms::Variant};
use pest::iterators::Pair;

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant<Term>, Error> {
    let mut inner = get_n_inner(p, vec!["Variant Label", "Variant Term", "Variant Type"])?;

    let var_pair = inner.remove(0);
    let var = var_pair.as_str();

    let term_pair = inner.remove(0);
    let variant_term = pair_to_term(term_pair)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let variant_ty = pair_to_type(ty_rule)?;

    Ok(Variant::new(var, variant_term, variant_ty))
}
