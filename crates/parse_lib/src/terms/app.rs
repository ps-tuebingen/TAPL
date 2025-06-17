use crate::{Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{App, Term};

impl<T> Parse for App<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;

    const RULE: Rule = Rule::term;

    fn from_pair(p: Pair<'_, Rule>, fun: Self::LeftRecArg) -> Result<Self, Self::ParseError> {
        let arg = T::from_pair(p, ())?;
        Ok(App::new(fun, arg))
    }
}
