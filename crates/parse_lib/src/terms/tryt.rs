use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Term, Try};

impl<T> Parse for Try<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    fn rule() -> Rule {
        Rule::try_term
    }

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Try<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Try Term", "With Term"])?;

        let tryt_rule = inner.remove(0);
        let try_t = T::from_pair(tryt_rule, ())?;
        let with_rule = inner.remove(0);
        let witht = T::from_pair(with_rule, ())?;

        Ok(Try::new(try_t, witht))
    }
}
