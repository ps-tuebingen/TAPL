use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{IsZero, Term};

impl<T> Parse for IsZero<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::iszero_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<IsZero<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["IsZero Argument"])?;
        let term_rule = inner.remove(0);
        let term_inner = pair_to_n_inner(term_rule, vec!["Prim Term Inner"])?.remove(0);
        let term = T::from_pair(term_inner, ())?;
        Ok(IsZero::new(term))
    }
}
