use crate::{pair_to_n_inner, to_paren_term_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{Pred, Term};

impl<T> Parse for Pred<T>
where
    T: Term + Parse,
{
    type ParseError = <T as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::pred_term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Pred<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(p, vec!["Pred Argument"])?;
        let arg_rule = to_paren_term_inner(inner.remove(0))?;
        let arg = T::from_pair(arg_rule)?;
        Ok(Pred::new(arg))
    }
}
