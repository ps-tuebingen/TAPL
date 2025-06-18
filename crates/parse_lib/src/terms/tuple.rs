use crate::{Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Term, Tuple};

impl<T> Parse for Tuple<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();
    const RULE: Rule = Rule::tuple_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Tuple<T>, Self::ParseError> {
        let mut terms = vec![];
        for p in p.into_inner() {
            let p_term = T::from_pair(p, ())?;
            terms.push(p_term);
        }
        Ok(Tuple::new(terms))
    }
}
