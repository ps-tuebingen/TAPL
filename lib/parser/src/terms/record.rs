use crate::{GroupParse, Parse, Rule};
use errors::{MissingInput, parse_error::ParserError};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::{language::Language, terms::Record};

impl<Lang> Parse for Record<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();

    const RULE: Rule = Rule::record_term;

    fn from_pair(p: Pair<'_, Rule>, (): Self::LeftRecArg) -> Result<Self, ParserError> {
        let mut inner = p.into_inner();
        let mut recs = HashMap::new();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let term_rule = inner.next().ok_or_else(|| {
                <MissingInput as Into<ParserError>>::into(MissingInput::new("Record Term"))
            })?;
            let term = Lang::Term::from_pair(term_rule, ())?;
            recs.insert(label, term);
        }
        Ok(Self::new(recs))
    }
}
