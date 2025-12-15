use crate::{GroupParse, Parse, Rule, pair_span, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Deref};

impl<Lang> Parse for Deref<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::deref_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let span = pair_span(&p);
        let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self::new(term, span))
    }
}
