use crate::{GroupParse, Parse, Rule};
use errors::{MissingInput, parse_error::ParserError};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::{language::Language, types::Record};

impl<Lang> Parse for Record<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::record_type;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut recs = HashMap::new();
        let mut inner = p.into_inner();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let ty_rule = inner.next().ok_or_else(|| {
                <MissingInput as Into<ParserError>>::into(MissingInput::new("Record Type"))
            })?;
            let ty = Lang::Type::from_pair(ty_rule, ())?;
            recs.insert(label, ty);
        }
        Ok(Self::new(recs))
    }
}
