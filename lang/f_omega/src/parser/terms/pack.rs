use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule};
use crate::syntax::terms::{Pack, Unpack};
use pest::iterators::Pair;

pub fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack, Error> {
    let mut inner = pair_to_n_inner(p, vec!["Pack Inner Type", "Packed Term", "Pack Outer Type"])?;
    let inner_rule = inner.remove(0);
    let inner_ty = pair_to_type(inner_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    let outer_rule = inner.remove(0);
    let outer_ty = pair_to_type(outer_rule)?;
    Ok(Pack {
        inner_ty,
        term: Box::new(term),
        outer_ty,
    })
}

pub fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Unpack Type Name",
            "Unpack Term Name",
            "Unpack Packed Term",
            "Unpack In Term",
        ],
    )?;
    let ty_name = inner.remove(0).as_str().trim().to_owned();
    let term_name = inner.remove(0).as_str().trim().to_owned();
    let packed_rule = inner.remove(0);
    let packed = pair_to_term(packed_rule)?;
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Unpack {
        ty_var: ty_name,
        bound_var: term_name,
        bound_term: Box::new(packed),
        in_term: Box::new(in_term),
    })
}
