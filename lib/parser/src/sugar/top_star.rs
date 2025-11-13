use crate::{GroupParse, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use std::marker::PhantomData;
use syntax::{language::Language, types::Top};

pub struct TopStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> TopStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    #[must_use]
    pub fn to_top(self) -> Top<Lang>
where {
        self.into()
    }
}

impl<Lang> Parse for TopStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type_star;

    fn from_pair(_: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(Self {
            phantom: PhantomData,
        })
    }
}

impl<Lang> From<TopStar<Lang>> for Top<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(_: TopStar<Lang>) -> Self {
        Self::new_star()
    }
}
