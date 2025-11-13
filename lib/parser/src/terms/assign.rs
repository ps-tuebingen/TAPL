use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Assign};

impl<Lang> Parse for Assign<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = Lang::Term;

    const RULE: Rule = Rule::assign;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Self, ParserError> {
        let term_rule = pair_to_n_inner(p, vec!["Assign Right hand side"])?.remove(0);
        let rhs = Lang::Term::from_pair(term_rule, ())?;
        Ok(Self::new(t, rhs))
    }
}
