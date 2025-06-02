use super::{pair_to_n_inner, Error, MissingInput, RemainingInput, Rule};
use common::parse::UnexpectedRule;
use pest::iterators::Pair;
use syntax::kinds::Kind;

pub fn pair_to_kind(p: Pair<'_, Rule>) -> Result<Kind, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner
        .next()
        .ok_or(MissingInput::new("Non Left-Recursive Kind"))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Kind"])?.remove(0);
    let prim_kind = pair_to_prim_kind(prim_inner)?;

    let kind = match inner.next() {
        None => prim_kind,
        Some(arrow) => pair_to_arrow_kind(arrow, prim_kind)?,
    };

    if let Some(n) = inner.next() {
        return Err(RemainingInput::new(&format!("{:?}", n.as_rule())));
    }

    Ok(kind)
}

fn pair_to_prim_kind(p: Pair<'_, Rule>) -> Result<Kind, Error> {
    match p.as_rule() {
        Rule::star_kind => Ok(Kind::Star),
        Rule::paren_kind => {
            let inner_rule = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
            pair_to_kind(inner_rule)
        }
        _ => Err(UnexpectedRule::new(p, "Non Left-Recursive Kind").into()),
    }
}

fn pair_to_arrow_kind(p: Pair<'_, Rule>, knd: Kind) -> Result<Kind, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Left Recursive Kind"])?.remove(0);
    let to_kind = pair_to_kind(to_rule)?;
    Ok(Kind::Arrow(Box::new(knd), Box::new(to_kind)))
}
