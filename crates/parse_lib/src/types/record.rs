use crate::{
    errors::{MissingInput, ParserError},
    Parse, Rule,
};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::{Record, Type};

impl<Ty> Parse for Record<Ty>
where
    Ty: Type + Parse,
{
    type ParseError = <Ty as Parse>::ParseError;

    fn rule() -> Rule {
        Rule::record_type
    }
    fn from_pair(p: Pair<'_, Rule>) -> Result<Record<Ty>, Self::ParseError> {
        let mut recs = HashMap::new();
        let mut inner = p.into_inner();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let ty_rule = inner
                .next()
                .ok_or(<MissingInput as Into<ParserError>>::into(
                    MissingInput::new("Record Type"),
                ))?;
            let ty = Ty::from_pair(ty_rule)?;
            recs.insert(label, ty);
        }
        Ok(Record::new(recs))
    }
}
