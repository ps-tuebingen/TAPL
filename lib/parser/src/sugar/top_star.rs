use crate::{GroupParse, ParsableLanguage, Parse, Rule};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use std::marker::PhantomData;
use syntax::types::Top;

pub struct TopStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> TopStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn to_top(self) -> Top<Lang>
where {
        self.into()
    }
}

impl<Lang> Parse for TopStar<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::top_type_star;

    fn from_pair(_: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<TopStar<Lang>, ParserError> {
        Ok(TopStar {
            phantom: PhantomData,
        })
    }
}

impl<Lang> From<TopStar<Lang>> for Top<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn from(_: TopStar<Lang>) -> Top<Lang> {
        Top::new_star()
    }
}
