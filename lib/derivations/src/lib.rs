use errors::UnexpectedDerivation;
use std::fmt;
use syntax::{terms::Term, types::Type};

pub mod conclusion;
pub mod definition_derivation;
pub mod program_derivation;
pub mod rules;
pub mod typing_derivation;

pub use conclusion::{Conclusion, SubtypeConclusion, TypingConclusion};
pub use definition_derivation::DefinitionDerivation;
pub use program_derivation::ProgramDerivation;
pub use rules::TypingRule;
pub use typing_derivation::TypingDerivation;

#[derive(Debug)]
pub enum Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    TypingDerivation(TypingDerivation<T, Ty>),
    ProgramDerivation(ProgramDerivation<T, Ty>),
    DefinitionDerivation(DefinitionDerivation<T, Ty>),
}

impl<T, Ty> Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn ret_ty(&self) -> Ty {
        match self {
            Derivation::TypingDerivation(deriv) => deriv.ret_ty(),
            Derivation::ProgramDerivation(deriv) => deriv.ret_ty(),
            Derivation::DefinitionDerivation(deriv) => deriv.ret_ty(),
        }
    }

    pub fn into_def(self) -> Result<DefinitionDerivation<T, Ty>, UnexpectedDerivation> {
        match self {
            Derivation::DefinitionDerivation(deriv) => Ok(deriv),
            Derivation::TypingDerivation(_) => Err(UnexpectedDerivation::new(
                "Typing Derivation",
                "Definition Derivation",
            )),
            Derivation::ProgramDerivation(_) => Err(UnexpectedDerivation::new(
                "Program Derivation",
                "Definition Derivation",
            )),
        }
    }

    pub fn into_ty(self) -> Result<TypingDerivation<T, Ty>, UnexpectedDerivation> {
        match self {
            Derivation::TypingDerivation(deriv) => Ok(deriv),
            Derivation::DefinitionDerivation(_) => Err(UnexpectedDerivation::new(
                "Definition Derivation",
                "Typing Derivation",
            )),
            Derivation::ProgramDerivation(_) => Err(UnexpectedDerivation::new(
                "Program Derivation",
                "Typing Derivation",
            )),
        }
    }

    pub fn into_prog(self) -> Result<ProgramDerivation<T, Ty>, UnexpectedDerivation> {
        match self {
            Derivation::ProgramDerivation(deriv) => Ok(deriv),
            Derivation::TypingDerivation(_) => Err(UnexpectedDerivation::new(
                "Typing Derivation",
                "Program Derivation",
            )),
            Derivation::DefinitionDerivation(_) => Err(UnexpectedDerivation::new(
                "Definition Derivation",
                "Program Derivation",
            )),
        }
    }
}

impl<T, Ty> From<TypingDerivation<T, Ty>> for Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(deriv: TypingDerivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation::TypingDerivation(deriv)
    }
}

impl<T, Ty> From<ProgramDerivation<T, Ty>> for Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(deriv: ProgramDerivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation::ProgramDerivation(deriv)
    }
}

impl<T, Ty> From<DefinitionDerivation<T, Ty>> for Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(deriv: DefinitionDerivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation::DefinitionDerivation(deriv)
    }
}

impl<T, Ty> fmt::Display for Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Derivation::TypingDerivation(deriv) => deriv.fmt(f),
            Derivation::ProgramDerivation(deriv) => deriv.fmt(f),
            Derivation::DefinitionDerivation(deriv) => deriv.fmt(f),
        }
    }
}
