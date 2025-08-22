use crate::{GroupParse, ParsableLanguage, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Fst;

impl<Lang> Parse for Fst<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::fst_term;

    fn from_pair(_: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(Fst::new(t))
    }
}
