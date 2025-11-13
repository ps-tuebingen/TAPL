use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> Parse for TryWithVal<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::try_catch;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Try Term", "Catch Term"])?;

        let tryt_rule = inner.remove(0);
        let tryt = Lang::Term::from_pair(tryt_rule, ())?;
        let catch_rule = inner.remove(0);
        let catch_term = Lang::Term::from_pair(catch_rule, ())?;
        Ok(Self::new(tryt, catch_term))
    }
}
