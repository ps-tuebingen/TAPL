use crate::{GroupParse, ParsableLanguage, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::types::List;

impl<Lang> Parse for List<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::list_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<List<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["List Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(List::new(ty))
    }
}
