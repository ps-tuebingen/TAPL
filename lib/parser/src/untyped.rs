use crate::{GroupParse, ParsableLanguage, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::untyped::Untyped;

impl<Lang> GroupParse for Untyped<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(_: Pair<'_, Rule>) -> Result<Self, ParserError> {
        Ok(Untyped::new())
    }

    fn from_pair_leftrec(_: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        Ok(Untyped::new())
    }
}
