use crate::{GroupParse, Parse, Rule, pair_to_n_inner};
use errors::parse_error::ParserError;
use pest::iterators::Pair;
use syntax::{language::Language, types::Sum};

impl<Lang> Parse for Sum<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::sum_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["First Sum Type", "Second Sum Type"])?;

        let fst_pair = inner.remove(0);
        let fst_ty = Lang::Type::from_pair(fst_pair, ())?;

        let snd_pair = inner.remove(0);
        let snd_ty = Lang::Type::from_pair(snd_pair, ())?;

        Ok(Self::new(fst_ty, snd_ty))
    }
}
