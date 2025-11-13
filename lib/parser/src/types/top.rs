use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    language::Language,
    {kinds::Kind, types::Top},
};

impl<Lang> Parse for Top<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let kind_rule = pair_to_n_inner(p, vec!["Kind"])?.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        Ok(Self::new(kind))
    }
}
