use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Unfold};

impl<Lang> Parse for Unfold<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::unfold_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Unfold<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Unfold Type", "Unfold Term"])?;
        let ty_rule = inner.remove(0);
        let ty = Lang::Type::from_pair(ty_rule, ())?;
        let term_rule = inner.remove(0);
        let term = Lang::Term::from_pair(term_rule, ())?;
        Ok(Unfold::new(ty, term))
    }
}
