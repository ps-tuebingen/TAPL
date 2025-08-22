use crate::{GroupParse, ParsableLanguage, Parse, Rule};

use errors::{MissingInput, parse_error::ParserError};
use pest::iterators::Pair;
use std::collections::HashMap;
use syntax::types::Variant;

impl<Lang> Parse for Variant<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    type LeftRecArg = ();
    const RULE: Rule = Rule::variant_type;

    fn from_pair(p: Pair<'_, Rule>, _: Self::LeftRecArg) -> Result<Variant<Lang>, ParserError> {
        let mut inner = p.into_inner();
        let mut variants = HashMap::new();
        while let Some(label_rule) = inner.next() {
            let label = label_rule.as_str().trim().to_owned();
            let ty_rule = inner
                .next()
                .ok_or(<MissingInput as Into<ParserError>>::into(
                    MissingInput::new("Variant Type"),
                ))?;
            let ty = Lang::Type::from_pair(ty_rule, ())?;
            variants.insert(label, ty);
        }
        Ok(Variant::new(variants))
    }
}
