use crate::{Name, language::Language, span::Span};
use macros::EqNoSpan;
use std::fmt;

#[derive(Debug, Clone, EqNoSpan)]
pub struct Definition<Lang>
where
    Lang: Language,
{
    pub name: Name,
    pub annot: Lang::Type,
    pub body: Lang::Term,
    pub span: Span,
}

impl<Lang> Definition<Lang>
where
    Lang: Language,
{
    pub fn new<T, Ty>(name: &str, annot: Ty, body: T, span: Span) -> Self
    where
        T: Into<Lang::Term>,
        Ty: Into<Lang::Type>,
    {
        Self {
            name: name.to_owned(),
            annot: annot.into(),
            body: body.into(),
            span,
        }
    }
}

impl<Lang> fmt::Display for Definition<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "def {}::{}:={};", self.name, self.annot, self.body)
    }
}
