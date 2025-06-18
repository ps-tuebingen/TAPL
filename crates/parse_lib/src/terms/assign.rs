use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Assign, Term};

impl<T> Parse for Assign<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;

    const RULE: Rule = Rule::assign;

    fn from_pair(p: Pair<'_, Rule>, t: Self::LeftRecArg) -> Result<Assign<T>, Self::ParseError> {
        let term_rule = pair_to_n_inner(p, vec!["Assign Right hand side"])?.remove(0);
        let rhs = T::from_pair(term_rule, ())?;
        Ok(Assign::new(t, rhs).into())
    }
}
