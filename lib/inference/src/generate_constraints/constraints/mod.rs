use syntax::language::Language;

mod equality;
mod subtyping;
pub use equality::EqualityConstraint;
pub use subtyping::SubtypeConstraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint<Lang>
where
    Lang: Language,
{
    Equality(EqualityConstraint<Lang>),
    Subtyping(SubtypeConstraint<Lang>),
}
