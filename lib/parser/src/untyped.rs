use crate::{GroupParse, Rule, pair_span};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, untyped::Untyped};

impl<Lang> GroupParse for Untyped<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
{
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        Ok(Self::new(span))
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, _: Self) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        Ok(Self::new(span))
    }
}
