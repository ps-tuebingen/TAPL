use crate::{errors::ParserError, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Fst, Term};

impl<T> Parse for Fst<T>
where
    T: Term + Parse,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::fst_term;

    fn from_pair(_: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(Fst::new(t))
    }
}
