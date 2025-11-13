use crate::{GroupParse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, untyped::Untyped};

impl<Lang> GroupParse for Untyped<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
{
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(_: Pair<'_, Rule>) -> Result<Self, ParserError> {
        Ok(Self::new())
    }

    fn from_pair_leftrec(_: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        Ok(Self::new())
    }
}
