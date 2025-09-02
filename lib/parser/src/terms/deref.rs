use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
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

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Deref<Lang>, ParserError> {
        let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Deref::new(term))
    }
}
