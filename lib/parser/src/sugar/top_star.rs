use crate::{GroupParse, Parse, Rule, pair_span};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use std::marker::PhantomData;
use syntax::{language::Language, span::Span, types::Top};

/// Helper struct for parsing [`syntax::types::top::Top`]
/// Allows writing `Top` instead of `Top[*]` in the source
pub struct TopStar<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Source Location
    pub span: Span,
    /// Save the type parameter
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

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        Ok(Self {
            span,
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
    fn from(ts: TopStar<Lang>) -> Self {
        Self::new_star(ts.span)
    }
}
