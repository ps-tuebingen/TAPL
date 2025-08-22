use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Something;

impl<Lang> Parse for Something<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::some_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Something<Lang>, ParserError> {
        let arg_pair = pair_to_n_inner(p, vec!["Something Argument"])?.remove(0);
        let arg = Lang::Term::from_pair(
            pair_to_n_inner(arg_pair, vec!["Something Inner"])?.remove(0),
            (),
        )?;
        Ok(Something::new(arg))
    }
}
