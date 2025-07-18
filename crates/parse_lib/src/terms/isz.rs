use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{IsZero, Term};

impl<T> Parse for IsZero<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::iszero_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<IsZero<T>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["IsZero Argument"])?;
        let term_rule = inner.remove(0);
        let term_inner = pair_to_n_inner(term_rule, vec!["Prim Term Inner"])?.remove(0);
        let term = T::from_pair(term_inner, ())?;
        Ok(IsZero::new(term))
    }
}
