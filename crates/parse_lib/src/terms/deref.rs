use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Deref, Term};

impl<T> Parse for Deref<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::deref_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Deref<T>, Self::ParseError> {
        let term_rule = pair_to_n_inner(p, vec!["Deref Term"])?.remove(0);
        let term = T::from_pair(term_rule, ())?;
        Ok(Deref::new(term))
    }
}
