use crate::{errors::ParserError, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{App, Term};

impl<T> Parse for App<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::term;

    fn from_pair(p: Pair<'_, Rule>, fun: Self::LeftRecArg) -> Result<Self, ParserError> {
        let arg = T::from_pair(p, ())?;
        Ok(App::new(fun, arg))
    }
}
