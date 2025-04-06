use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::terms::{Pack, Unpack};
use pest::iterators::Pair;

pub fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Packed Type", "Packed Term", "Pack Outer Type"])?;
    let packed_rule = inner.remove(0);
    let packed = pair_to_type(packed_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    let outer_rule = inner.remove(0);
    let outer = pair_to_type(outer_rule)?;
    Ok(Pack {
        inner_ty: packed,
        term: Box::new(term),
        outer_ty: outer,
    })
}

pub fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Let Keyword",
            "Unpacked type Name",
            "Unpacked Term Name",
            "Unpack Packed Term",
            "In Keyword",
            "Unpack In Term",
        ],
    )?;
    inner.remove(0);
    let ty_name = inner.remove(0).as_str().trim().to_owned();
    let term_name = inner.remove(0).as_str().trim().to_owned();
    let packed_rule = inner.remove(0);
    let packed = pair_to_term(packed_rule)?;
    inner.remove(0);
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Unpack {
        ty_var: ty_name,
        bound_var: term_name,
        bound_term: Box::new(packed),
        in_term: Box::new(in_term),
    })
}
