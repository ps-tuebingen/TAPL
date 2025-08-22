use crate::{definition::Definition, language::Language};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DefinitionValue<Lang>
where
    Lang: Language,
{
    name: String,
    annot: Lang::Type,
    body: Lang::Value,
}

impl<Lang> From<DefinitionValue<Lang>> for Definition<Lang>
where
    Lang: Language,
{
    fn from(def: DefinitionValue<Lang>) -> Definition<Lang> {
        Definition {
            name: def.name,
            annot: def.annot,
            body: def.body.into(),
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
