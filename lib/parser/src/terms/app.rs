use crate::{GroupParse, ParsableLanguage, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::App;

impl<Lang> Parse for App<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::term;

    fn from_pair(p: Pair<'_, Rule>, fun: Self::LeftRecArg) -> Result<Self, ParserError> {
        let arg = Lang::Term::from_pair(p, ())?;
        Ok(App::new(fun, arg))
    }
}
