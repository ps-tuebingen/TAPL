use crate::{GroupParse, Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{language::Language, types::Nat};

impl<Lang> Parse for Nat<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let nat = Self::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == nat.to_string().to_lowercase() {
            Ok(nat)
        } else {
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
