use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Nothing;

impl<Lang> Parse for Nothing<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::none_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Nothing<Lang>, ParserError> {
        let ty_pair = pair_to_n_inner(p, vec!["Nothing Type"])?.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;
        Ok(Nothing::new(ty))
    }
}
