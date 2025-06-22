use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Fix, Term};

impl<T> Parse for Fix<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::fix_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Fix<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Fix Term"])?;
        let inner_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let inner = T::from_pair(inner_rule, ())?;
        Ok(Fix::new(inner))
    }
}
