use crate::{GroupParse, ParsableLanguage, Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::types::Unit;

impl<Lang> Parse for Unit<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unit<Lang>, ParserError> {
        let u = Unit::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == u.to_string().to_lowercase() {
            Ok(u)
        } else {
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
