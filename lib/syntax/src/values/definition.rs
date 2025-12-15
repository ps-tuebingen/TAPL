use crate::{definition::Definition, language::Language, span::Span};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DefinitionValue<Lang>
where
    Lang: Language,
{
    name: String,
    annot: Lang::Type,
    body: Lang::Value,
    span: Span,
}

impl<Lang> From<DefinitionValue<Lang>> for Definition<Lang>
where
    Lang: Language,
{
    fn from(def: DefinitionValue<Lang>) -> Self {
        Self {
            name: def.name,
            annot: def.annot,
            body: def.body.into(),
            span: def.span,
        }
    }
}

impl<Lang> fmt::Display for DefinitionValue<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}:={}", self.name, self.annot, self.body)
    }
}
