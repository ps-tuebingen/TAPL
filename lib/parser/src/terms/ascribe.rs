use crate::{GroupParse, Parse, Rule, pair_span, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, span::Spanned, terms::Ascribe};

impl<Lang> Parse for Ascribe<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::ascription;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p).extend(&t.span());
        let mut inner = pair_to_n_inner(p, vec!["Ascribed Type"])?;

        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Self::new(t, ty, span))
    }
}
