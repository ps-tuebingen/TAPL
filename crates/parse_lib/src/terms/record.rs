use crate::{
    errors::{MissingInput, ParserError},
    Parse, Rule,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::terms::{Record, Term};

impl<T> Parse for Record<T>
where
    T: Term + Parse,
{
    type ParseError = <T as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::record_term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Record<T>, Self::ParseError> {
        let mut inner = p.into_inner();
        let mut recs = HashMap::new();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let term_rule = inner
                .next()
                .ok_or(<MissingInput as Into<ParserError>>::into(
                    MissingInput::new("Record Term"),
                ))?;
            let term = T::from_pair(term_rule)?;
            recs.insert(label, term);
        }
        Ok(Record::new(recs))
    }
}
