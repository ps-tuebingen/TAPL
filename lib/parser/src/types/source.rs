use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::Source};

impl<Lang> Parse for Source<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::source_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Source<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Source Type"])?;
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        Ok(Source::new(ty))
    }
}
