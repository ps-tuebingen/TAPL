use crate::{GroupParse, ParsableLanguage, Parse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::types::Bool;

impl<Lang> Parse for Bool<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::const_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Bool<Lang>, ParserError> {
        let bl = Bool::new();
        let p_str = p.as_str().trim().to_lowercase();
        if p_str == bl.to_string().to_lowercase() {
            Ok(bl)
        } else {
            Err(UnknownKeyword::new(&p_str).into())
        }
    }
}
