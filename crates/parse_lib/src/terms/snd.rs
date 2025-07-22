use crate::{Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::{Snd, Term};

impl<T> Parse for Snd<T>
where
    T: Term + Parse,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::fst_term;

    fn from_pair(_: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(Snd::new(t))
    }
}
