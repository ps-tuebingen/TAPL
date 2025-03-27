use super::{pair_to_n_inner, Error, Rule};
use crate::kinds::Kind;
use pest::iterators::Pair;

pub fn pair_to_kind(p: Pair<'_, Rule>) -> Result<Kind, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner.next().ok_or(Error::MissingInput("Kind".to_owned()))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Kind"])?.remove(0);
    let prim_kind = pair_to_prim_kind(prim_inner)?;

    let kind = match inner.next() {
        None => prim_kind,
        Some(leftrec) => {
            let leftrec_inner = pair_to_n_inner(leftrec, vec!["Left Recursive Type"])?.remove(0);
            pair_to_leftrec_kind(leftrec_inner, prim_kind)?
        }
    };

    if let Some(n) = inner.next() {
        return Err(Error::RemainingInput(n.as_rule()));
    }
    Ok(kind)
}

fn pair_to_prim_kind(p: Pair<'_, Rule>) -> Result<Kind, Error> {
    match p.as_rule() {
        Rule::kind_star => Ok(Kind::Star),
        Rule::paren_kind => {
            let inner = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
            pair_to_kind(inner)
        }
        r => Err(Error::unexpected(r, "Kind")),
    }
}

fn pair_to_leftrec_kind(p: Pair<'_, Rule>, knd: Kind) -> Result<Kind, Error> {
    let inner = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
    let to_kind = pair_to_kind(inner)?;
    Ok(Kind::Arrow(Box::new(knd), Box::new(to_kind)))
}
