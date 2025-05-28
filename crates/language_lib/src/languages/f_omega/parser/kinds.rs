use super::{pair_to_n_inner, to_parse_err, Rule};
use common::errors::{Error, ErrorKind};
use pest::iterators::Pair;
use syntax::kinds::Kind;

pub fn pair_to_kind(p: Pair<'_, Rule>) -> Result<Kind, Error> {
    let mut inner = p.into_inner();
    let prim_rule = inner.next().ok_or(to_parse_err(ErrorKind::MissingInput(
        "Non Left-Recursive Kind".to_owned(),
    )))?;
    let prim_inner = pair_to_n_inner(prim_rule, vec!["Non Left-Recursive Kind"])?.remove(0);
    let prim_kind = pair_to_prim_kind(prim_inner)?;

    let kind = match inner.next() {
        None => prim_kind,
        Some(arrow) => pair_to_arrow_kind(arrow, prim_kind)?,
    };

    if let Some(n) = inner.next() {
        return Err(to_parse_err(ErrorKind::RemainingInput(format!("{n:?}"))));
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
        _ => Err(to_parse_err(ErrorKind::UnexpectedRule {
            found: format!("{p:?}"),
            expected: "Non Left-Recursive Kind".to_owned(),
        })),
    }
}

fn pair_to_arrow_kind(p: Pair<'_, Rule>, knd: Kind) -> Result<Kind, Error> {
    let to_rule = pair_to_n_inner(p, vec!["Left Recursive Kind"])?.remove(0);
    let to_kind = pair_to_kind(to_rule)?;
    Ok(Kind::Arrow(Box::new(knd), Box::new(to_kind)))
}
