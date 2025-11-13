use crate::{GroupParse, Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{language::Language, types::Bool};

impl<Lang> Parse for Bool<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let bl = Self::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == bl.to_string().to_lowercase() {
            Ok(bl)
        } else {
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
