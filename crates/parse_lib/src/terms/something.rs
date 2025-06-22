use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Something, Term};

impl<T> Parse for Something<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::some_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Something<T>, Self::ParseError> {
        let arg_pair = pair_to_n_inner(p, vec!["Something Argument"])?.remove(0);
        let arg = T::from_pair(
            pair_to_n_inner(arg_pair, vec!["Something Inner"])?.remove(0),
            (),
        )?;
        Ok(Something::new(arg))
    }
}
