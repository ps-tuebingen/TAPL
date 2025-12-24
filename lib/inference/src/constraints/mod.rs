use syntax::language::Language;

mod equality;
mod indexing;
mod kinding;
mod record;
mod subtyping;
mod variant;
pub use equality::EqualityConstraint;
pub use indexing::IndexConstraint;
pub use kinding::{KindConstraint, KindOrVar};
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
