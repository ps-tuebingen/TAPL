use crate::{GroupParse, Parse, Rule, pair_span};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, span::Spanned, terms::Fst};

impl<Lang> Parse for Fst<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::fst_term;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p).extend(&t.span());
        Ok(Self::new(t, span))
    }
}
