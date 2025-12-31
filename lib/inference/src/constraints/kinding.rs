use super::Constraint;
use syntax::{kinds::Kind, language::Language, span::Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindConstraint {
    pub left: Kind,
    pub right: Kind,
    pub span: Span,
}

impl KindConstraint {
    pub fn new<K1, K2>(left: K1, right: K2, span: Span) -> Self
    where
        K1: Into<Kind>,
        K2: Into<Kind>,
    {
        Self {
            left: left.into(),
            right: right.into(),
            span,
        }
    }
}

impl<Lang> From<KindConstraint> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(c: KindConstraint) -> Self {
        Constraint::Kinding(c)
    }
}
