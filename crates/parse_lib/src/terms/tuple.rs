use crate::{errors::ParserError, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Term, Tuple};

impl<T> Parse for Tuple<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::tuple_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tuple<T>, ParserError> {
        let mut terms = vec![];
        for p in p.into_inner() {
            let p_term = T::from_pair(p, ())?;
            terms.push(p_term);
        }
        Ok(Tuple::new(terms))
    }
}
