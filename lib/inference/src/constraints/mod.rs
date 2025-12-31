use std::fmt;
use syntax::language::Language;

mod definition;
mod equality;
mod indexing;
mod kinding;
mod program;
mod record;
mod subtyping;
mod variant;
pub use definition::DefConstraints;
pub use equality::EqualityConstraint;
pub use indexing::IndexConstraint;
pub use kinding::KindConstraint;
pub use program::ProgramConstraints;
pub use record::RecordConstraint;
pub use subtyping::SubtypeConstraint;
pub use variant::VariantConstraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint<Lang>
where
    Lang: Language,
{
    Equality(EqualityConstraint<Lang>),
    Subtyping(SubtypeConstraint<Lang>),
    Kinding(KindConstraint),
    Indexing(IndexConstraint<Lang>),
    Record(RecordConstraint<Lang>),
    Variant(VariantConstraint<Lang>),
}

impl<Lang> fmt::Display for Constraint<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Equality(eq) => eq.fmt(f),
            Self::Subtyping(sub) => sub.fmt(f),
            Self::Kinding(knd) => knd.fmt(f),
            Self::Indexing(ind) => ind.fmt(f),
            Self::Record(rec) => rec.fmt(f),
            Self::Variant(var) => var.fmt(f),
        }
    }
}
