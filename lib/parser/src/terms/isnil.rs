use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::IsNil};

impl<Lang> Parse for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::isnil_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["IsNil Type", "IsNil Argument"])?;

        let ty_pair = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_pair, ())?;

        let term_pair = inner.remove(0);
        let term = Lang::Term::from_pair(term_pair, ())?;

        Ok(Self::new(term, ty))
    }
}
