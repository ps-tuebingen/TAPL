use crate::{pair_to_n_inner, Parse, Rule};
use pest::iterators::Pair;
use syntax::terms::{SomeCase, Term};

impl<T> Parse for SomeCase<T>
where
    T: Term + Parse<LeftRecArg = ()>,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = ();
    const RULE: Rule = Rule::somecase_term;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<SomeCase<T>, Self::ParseError> {
        let mut inner = pair_to_n_inner(
            p,
            vec!["Case Bound term", "Some/None Pattern", "Some/None Pattern"],
        )?;
        let bound_rule = inner.remove(0);
        let bound_term = T::from_pair(bound_rule, ())?;
        let fst_pt = inner.remove(0);
        let snd_pt = inner.remove(0);
        let (some_rule, none_rule) = match fst_pt.as_rule() {
            Rule::some_pattern => (fst_pt, snd_pt),
            _ => (snd_pt, fst_pt),
        };
        let none_inner = pair_to_n_inner(none_rule, vec!["None Term"])?.remove(0);
        let none_term = T::from_pair(none_inner, ())?;
        let mut some_inner = pair_to_n_inner(some_rule, vec!["Some Variable", "Some Term"])?;
        let some_var = some_inner.remove(0).as_str().to_owned();
        let some_term_rule = some_inner.remove(0);
        let some_term = T::from_pair(some_term_rule, ())?;
        Ok(SomeCase::new(bound_term, none_term, &some_var, some_term))
    }
}
