use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term};
use common::terms::{Pack, Unpack};
use pest::iterators::Pair;

pub fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack<Term>, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pack Inner Type", "Packed Term", "Pack Outer Type"])?;
    let inner_rule = inner.remove(0);
    let inner_ty = pair_to_type(inner_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    let outer_rule = inner.remove(0);
    let outer_ty = pair_to_type(outer_rule)?;
    Ok(Pack::new(inner_ty, term, outer_ty))
}

pub fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack<Term>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Unpack Type Name",
            "Unpack Term Name",
            "Unpack Packed Term",
            "Unpack In Term",
        ],
    )?;
    let ty_name = inner.remove(0).as_str().trim();
    let term_name = inner.remove(0).as_str().trim();
    let packed_rule = inner.remove(0);
    let packed = pair_to_term(packed_rule)?;
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Unpack::new(ty_name, term_name, packed, in_term))
}
