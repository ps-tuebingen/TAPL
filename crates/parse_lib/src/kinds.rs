use crate::{
    Parse, Rule,
    errors::{MissingInput, ParserError, RemainingInput, UnexpectedRule},
    pair_to_n_inner,
};
use pest::iterators::Pair;
use syntax::kinds::Kind;

impl Parse for Kind {
    type ParseError = ParserError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::kind;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Kind, Self::ParseError> {
        let mut inner = p.into_inner();
        let prim_rule = inner.next().ok_or(MissingInput::new("Kind"))?;
        let prim_inner = pair_to_n_inner(prim_rule, vec!["Kind"])?.remove(0);
        let prim_kind = pair_to_prim_kind(prim_inner)?;

        let kind = match inner.next() {
            None => prim_kind,
            Some(leftrec) => pair_to_leftrec_kind(leftrec, prim_kind)?,
        };

        if let Some(n) = inner.next() {
            return Err(RemainingInput::new(&format!("{:?}", n.as_rule())).into());
        }
        Ok(kind)
    }
}

fn pair_to_prim_kind(p: Pair<'_, Rule>) -> Result<Kind, ParserError> {
    match p.as_rule() {
        Rule::star_kind => Ok(Kind::Star),
        Rule::paren_kind => Kind::from_pair(pair_to_n_inner(p, vec!["Kind"])?.remove(0), ()),
        r => Err(UnexpectedRule::new(r, "Kind").into()),
    }
}

fn pair_to_leftrec_kind(p: Pair<'_, Rule>, knd: Kind) -> Result<Kind, ParserError> {
    let inner = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
    let to_kind = Kind::from_pair(inner, ())?;
    Ok(Kind::Arrow(Box::new(knd), Box::new(to_kind)))
}
