use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{
    language::Language,
    {kinds::Kind, types::Forall},
};

impl<Lang> Parse for Forall<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::forall_unbounded_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Forall Variable", "Forall Kind", "Forall Body"])?;
        let var = pair_to_n_inner(inner.remove(0), vec!["Forall Variable"])?
            .remove(0)
            .as_str()
            .trim();
        let kind_rule = inner.remove(0);
        let kind = Kind::from_pair(kind_rule, ())?;
        let body_rule = inner.remove(0);
        let body = Lang::Type::from_pair(body_rule, ())?;
        Ok(Self::new(var, kind, body))
    }
}
