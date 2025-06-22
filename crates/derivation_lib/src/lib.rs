use syntax::{terms::Term, types::Type};

pub mod conclusion;
pub mod definition_derivation;
pub mod program_derivation;
pub mod rules;
pub mod typing_derivation;

pub use conclusion::Conclusion;
pub use definition_derivation::DefinitionDerivation;
pub use program_derivation::ProgramDerivation;
pub use rules::TypingRule;
pub use typing_derivation::TypingDerivation;

pub trait Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> Derivation<T, Ty> for TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
impl<T, Ty> Derivation<T, Ty> for ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
impl<T, Ty> Derivation<T, Ty> for DefinitionDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
