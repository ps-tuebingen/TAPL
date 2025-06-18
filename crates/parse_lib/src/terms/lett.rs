use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Let, Term};

impl<T> Parse for Let<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::let_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Let<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Let Variable", "Let Bound Term", "Let In Term"])?;
        let var = inner.remove(0).as_str().trim();
        let bound_rule = inner.remove(0);
        let bound_term = T::from_pair(bound_rule, ())?;
        let in_rule = inner.remove(0);
        let in_term = T::from_pair(in_rule, ())?;
        Ok(Let::new(var, bound_term, in_term))
    }
}
