/// A Source Position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Source Line
    pub line: u64,
    /// Source Char / Column
    pub char: u64,
}

/// A Source Span
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Start position
    pub start: Position,
    /// End position
    pub end: Position,
}

impl Span {
    /// Extend `self` with `other`
    /// The new span will be `self.start` to `other.end`
    pub fn extend(&self, other: &Span) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }
}

/// Trait for anything that has a source span
pub trait Spanned {
    /// Get the span of `self`
    fn span(&self) -> Span;
}
