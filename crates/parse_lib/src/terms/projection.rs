use crate::{Parse, Rule, pair_to_n_inner};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::terms::{Projection, Term};

impl<T> Parse for Projection<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = T;

    const RULE: Rule = Rule::projection;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Projection<T>, ParserError> {
        let num_rule = pair_to_n_inner(p, vec!["Projection Index"])?.remove(0);
        let num = num_rule.as_str().trim().parse::<usize>().map_err(|_| {
            <UnknownKeyword as Into<ParserError>>::into(UnknownKeyword::new(num_rule.as_str()))
        })?;
        Ok(Projection::new(t, num))
    }
}
