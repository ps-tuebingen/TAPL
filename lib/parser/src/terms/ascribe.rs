use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::terms::Ascribe;

impl<Lang> Parse for Ascribe<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::ascription;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Ascribe<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Ascribed Type"])?;

        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;

        Ok(Ascribe::new(t, ty))
    }
}
