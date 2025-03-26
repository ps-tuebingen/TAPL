use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::syntax::{Pack, Unpack};
use pest::iterators::Pair;

pub fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec!["Packed Type", "Packed Term", "As Keyword", "Pack Type"],
    )?;
    let packed_rule = inner.remove(0);
    let packed_ty = pair_to_type(packed_rule)?;

    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;

    inner.remove(0);
    let pack_rule = inner.remove(0);
    let pack_ty = pair_to_type(pack_rule)?;

    Ok(Pack {
        inner_ty: packed_ty,
        term: Box::new(term),
        outer_ty: pack_ty,
    })
}

pub fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Let Keyword",
            "Unpack Type Name",
            "Unpack Term Name",
            "Pack Term",
            "In Keyword",
            "Unpack Term",
        ],
    )?;
    inner.remove(0);
    let ty_name = inner.remove(0).as_str().trim().to_owned();
    let term_name = inner.remove(0).as_str().trim().to_owned();
    let pack_rule = inner.remove(0);
    let pack_term = pair_to_term(pack_rule)?;

    inner.remove(0);
    let unpack_rule = inner.remove(0);
    let unpack_term = pair_to_term(unpack_rule)?;
    Ok(Unpack {
        ty_var: ty_name,
        bound_var: term_name,
        bound_term: Box::new(pack_term),
        in_term: Box::new(unpack_term),
    })
}
