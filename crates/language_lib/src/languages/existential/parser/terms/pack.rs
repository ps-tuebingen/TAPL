use super::{pair_to_n_inner, pair_to_term, pair_to_type, Error, Rule, Term, Type};
use pest::iterators::Pair;
use syntax::terms::{Pack, Unpack};

pub fn pair_to_pack(p: Pair<'_, Rule>) -> Result<Pack<Term, Type>, Error> {
    let mut inner = pair_to_n_inner(
        p,
        vec![
            "Packed Type",
            "Packed Term",
            "As Keyword",
            "Pack Outer Type",
        ],
    )?;
    let packed_rule = inner.remove(0);
    let packed = pair_to_type(packed_rule)?;
    let term_rule = inner.remove(0);
    let term = pair_to_term(term_rule)?;
    inner.remove(0);
    let outer_rule = inner.remove(0);
    let outer = pair_to_type(outer_rule)?;
    Ok(Pack::new(packed, term, outer))
}

pub fn pair_to_unpack(p: Pair<'_, Rule>) -> Result<Unpack<Term, Type>, Error> {
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
    let ty_name = inner.remove(0).as_str().trim();
    let term_name = inner.remove(0).as_str().trim();
    let packed_rule = inner.remove(0);
    let packed = pair_to_term(packed_rule)?;
    inner.remove(0);
    let in_rule = inner.remove(0);
    let in_term = pair_to_term(in_rule)?;
    Ok(Unpack::new(ty_name, term_name, packed, in_term))
}
