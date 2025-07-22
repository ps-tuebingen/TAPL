use crate::{GroupParse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::untyped::Untyped;

impl GroupParse for Untyped {
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(_: Pair<'_, Rule>) -> Result<Self, ParserError> {
        Ok(Untyped)
    }

    fn from_pair_leftrec(_: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        Ok(Untyped)
    }
}
