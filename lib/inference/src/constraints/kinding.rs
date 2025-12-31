use super::Constraint;
use std::fmt;
use syntax::{kinds::Kind, language::Language, span::Span};

/// Equality Constraint between kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindConstraint {
    /// Left kind
    pub left: Kind,
    /// Right kind
    pub right: Kind,
    /// Source location
    pub span: Span,
}

impl KindConstraint {
    /// Create a new constraint from given kinds
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
        Self::Kinding(c)
    }
}

impl fmt::Display for KindConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} == {}", self.left, self.right)
    }
}
