use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Term, Try};

impl<T> Parse for Try<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::try_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Try<T>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Try Term", "With Term"])?;

        let tryt_rule = inner.remove(0);
        let try_t = T::from_pair(tryt_rule, ())?;
        let with_rule = inner.remove(0);
        let witht = T::from_pair(with_rule, ())?;

        Ok(Try::new(try_t, witht))
    }
}
