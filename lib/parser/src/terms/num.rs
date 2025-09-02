use crate::{GroupParse, Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{language::Language, terms::Num};

impl<Lang> Parse for Num<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::number;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let num = p.as_str().trim().parse::<i64>().map_err(|_| {
            <UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(p.as_str()))
        })?;
        Ok(Num::new(num))
    }
}
