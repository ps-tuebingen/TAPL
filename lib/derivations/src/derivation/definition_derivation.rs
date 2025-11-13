use super::TypingDerivation;
use std::fmt;
use syntax::language::Language;

#[derive(Debug)]
pub struct DefinitionDerivation<Lang>
where
    Lang: Language,
{
    pub name: String,
    pub body_derivation: TypingDerivation<Lang>,
}

impl<Lang> DefinitionDerivation<Lang>
where
    Lang: Language,
{
    pub fn new(name: &str, body_derivation: TypingDerivation<Lang>) -> Self {
        Self {
            name: name.to_owned(),
            body_derivation,
        }
    }

    pub fn ret_ty(&self) -> Lang::Type {
        self.body_derivation.ret_ty()
    }
}

impl<Lang> fmt::Display for DefinitionDerivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.body_derivation)
    }
}
