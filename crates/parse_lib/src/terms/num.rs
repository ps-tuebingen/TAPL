use crate::{
    errors::{ParserError, UnknownKeyword},
    Parse, Rule,
};
use pest::iterators::Pair;
use syntax::terms::{Num, Term};

impl<T> Parse for Num<T>
where
    T: Term + Parse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::number;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Self, ParserError> {
        let num = p.as_str().trim().parse::<i64>().map_err(|_| {
            <UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(p.as_str()))
        })?;
        Ok(Num::new(num))
    }
}
