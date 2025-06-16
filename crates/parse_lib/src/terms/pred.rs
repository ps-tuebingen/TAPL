use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Pred, Term};

impl<T> Parse for Pred<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::pred_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Pred<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Pred Argument"])?;
        let arg_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let arg = T::from_pair(arg_rule, ())?;
        Ok(Pred::new(arg))
    }
}
