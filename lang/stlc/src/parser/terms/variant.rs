use super::pair_to_term;
use crate::{
    parser::{errors::Error, get_n_inner, next_rule, types::pair_to_type, Rule},
    syntax::Variant,
};
use pest::iterators::Pair;

pub fn pair_to_variant(p: Pair<'_, Rule>) -> Result<Variant, Error> {
    println!("trying to parse variant");
    let mut inner = get_n_inner(p, vec!["Variant Label", "Variant Term", "Variant Type"])?;

    let var_pair = inner.remove(0);
    let var = var_pair.as_str().to_owned();

    let term_pair = inner.remove(0);
    let variant_term = pair_to_term(term_pair)?;

    let ty_pair = inner.remove(0);
    let ty_rule = next_rule(ty_pair, Rule::r#type)?;
    let variant_ty = pair_to_type(ty_rule)?;

    Ok(Variant {
        label: var,
        term: Box::new(variant_term),
        ty: variant_ty,
    })
}
