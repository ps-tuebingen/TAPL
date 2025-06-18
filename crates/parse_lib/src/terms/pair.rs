use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Pair as PairT, Term};

impl<T> Parse for PairT<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::pair_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<PairT<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Pair First", "Pair Second"])?;
        let fst_rule = inner.remove(0);
        let fst = T::from_pair(fst_rule, ())?;
        let snd_rule = inner.remove(0);
        let snd = T::from_pair(snd_rule, ())?;
        Ok(PairT::new(fst, snd))
    }
}
