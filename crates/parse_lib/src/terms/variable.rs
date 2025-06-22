use crate::{Parse, Rule, errors::ParserError};
use pest::iterators::Pair;
use syntax::terms::{Term, Variable};

impl<T> Parse for Variable<T>
where
    T: Term + Parse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::variable;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        Ok(Variable::new(p.as_str().trim()))
    }
}
