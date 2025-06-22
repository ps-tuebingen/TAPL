use crate::{Parse, Rule, pair_to_n_inner};
use pest::iterators::Pair;
use syntax::terms::{RecordProj, Term};

impl<T> Parse for RecordProj<T>
where
    T: Term + Parse,
{
    type ParseError = <T as Parse>::ParseError;
    type LeftRecArg = T;

    const RULE: Rule = Rule::record_proj;

    fn from_pair(
        p: Pair<'_, Rule>,
        t: Self::LeftRecArg,
    ) -> Result<RecordProj<T>, Self::ParseError> {
        let label = pair_to_n_inner(p, vec!["Projection Target"])?
            .remove(0)
            .as_str()
            .trim();
        Ok(RecordProj::new(t, label))
    }
}
