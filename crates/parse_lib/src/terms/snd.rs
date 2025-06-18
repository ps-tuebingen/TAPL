use crate::{Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Snd, Term};

impl<T> Parse for Snd<T>
where
    T: Term + Parse,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;

    const RULE: Rule = Rule::fst_term;

    fn from_pair(_: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        Ok(Snd::new(t))
    }
}
