use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Term, TryWithVal};

impl<T> Parse for TryWithVal<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::try_catch;

    fn from_pair(
        p: Pair<'_, Rule>,
        _: Self::LeftRecArg,
    ) -> Result<TryWithVal<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Try Term", "Catch Term"])?;

        let tryt_rule = inner.remove(0);
        let tryt = T::from_pair(tryt_rule, ())?;
        let catch_rule = inner.remove(0);
        let catch_term = T::from_pair(catch_rule, ())?;
        Ok(TryWithVal::new(tryt, catch_term))
    }
}
