use crate::{pair_to_n_inner, to_paren_term_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Succ, Term};

impl<T> Parse for Succ<T>
where
    T: Term + Parse,
{
    type ParseError = <T as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::succ_term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Succ<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Succ Argument"])?;
        let arg_rule = to_paren_term_inner(inner.remove(0))?;
        let arg_term = T::from_pair(arg_rule)?;
        Ok(Succ::new(arg_term))
    }
}
