use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Let};

impl<Lang> Parse for Let<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::let_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Let Variable", "Let Bound Term", "Let In Term"])?;
        let var = inner.remove(0).as_str().trim();
        let bound_rule = inner.remove(0);
        let bound_term = Lang::Term::from_pair(bound_rule, ())?;
        let in_rule = inner.remove(0);
        let in_term = Lang::Term::from_pair(in_rule, ())?;
        Ok(Self::new(var, bound_term, in_term))
    }
}
