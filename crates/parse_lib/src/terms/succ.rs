use crate::{Parse, Rule, errors::ParserError, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Succ, Term};

impl<T> Parse for Succ<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::succ_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Succ<T>, ParserError> {
        let mut inner = pair_to_n_inner(p, vec!["Succ Argument"])?;
        let arg_rule = pair_to_n_inner(inner.remove(0), vec!["Prim Term Inner"])?.remove(0);
        let arg_term = T::from_pair(arg_rule, ())?;
        Ok(Succ::new(arg_term))
    }
}
