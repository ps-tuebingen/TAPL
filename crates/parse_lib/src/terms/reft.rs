use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{Ref, Term};

impl<T> Parse for Ref<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();

    const RULE: Rule = Rule::ref_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Ref<T>, Self::ParseError> {
        let mut inner_rules = pair_to_n_inner(p, vec!["Ref Term"])?;
        let term_rule = inner_rules.remove(0);
        let term = T::from_pair(
            pair_to_n_inner(term_rule, vec!["Ref Argument"])?.remove(0),
            (),
        )?;
        Ok(Ref::new(term))
    }
}
