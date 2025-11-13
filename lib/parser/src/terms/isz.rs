use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::IsZero};

impl<Lang> Parse for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::iszero_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["IsZero Argument"])?;
        let term_rule = inner.remove(0);
        let term_inner = pair_to_n_inner(term_rule, vec!["Prim Term Inner"])?.remove(0);
        let term = Lang::Term::from_pair(term_inner, ())?;
        Ok(Self::new(term))
    }
}
