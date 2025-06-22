use crate::{
    Parse, Rule,
    errors::{MissingInput, ParserError, RemainingInput},
};
use pest::iterators::Pair;
use syntax::{
    terms::{Tail, Term},
    types::Type,
};

impl<T, Ty> Parse for Tail<T, Ty>
where
    T: Term + Parse<LeftRecArg = ()>,
    Ty: Type + Parse<LeftRecArg = ()>,
    <T as Parse>::ParseError: From<<Ty as Parse>::ParseError>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::tail_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tail<T, Ty>, Self::ParseError> {
        let mut inner = p.into_inner();
        let ty_rule = inner
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new("Head Type"),
            ))?;
        let ty = Ty::from_pair(ty_rule, ())?;

        let term_pair = inner
            .next()
            .ok_or(<MissingInput as Into<ParserError>>::into(
                MissingInput::new("Head Argument"),
            ))?;
        let term = T::from_pair(term_pair, ())?;

        if let Some(next) = inner.next() {
            return Err(
                <RemainingInput as Into<ParserError>>::into(RemainingInput::new(&format!(
                    "{:?}",
                    next.as_rule()
                )))
                .into(),
            );
        }
        Ok(Tail::new(term, ty))
    }
}
