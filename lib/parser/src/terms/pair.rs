use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, terms::Pair as PairT};

impl<Lang> Parse for PairT<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::pair_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<PairT<Lang>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Pair First", "Pair Second"])?;
        let fst_rule = inner.remove(0);
        let fst = Lang::Term::from_pair(fst_rule, ())?;
        let snd_rule = inner.remove(0);
        let snd = Lang::Term::from_pair(snd_rule, ())?;
        Ok(PairT::new(fst, snd))
    }
}
