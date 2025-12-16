use crate::{GroupParse, Parse, Rule, pair_span};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::TypeVariable};

impl<Lang> Parse for TypeVariable<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::type_variable;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        Ok(Self::new(p.as_str().trim(), span))
    }
}
