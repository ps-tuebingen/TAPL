use crate::{GroupParse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{terms::Term, untyped::Untyped};

impl<T> GroupParse for Untyped<T>
where
    T: Term,
{
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(_: Pair<'_, Rule>) -> Result<Self, ParserError> {
        Ok(Untyped::new())
    }

    fn from_pair_leftrec(_: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        Ok(Untyped::new())
    }
}
