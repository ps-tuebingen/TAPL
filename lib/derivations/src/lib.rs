use errors::UnexpectedDerivation;
use std::fmt;
use syntax::language::Language;

pub mod conclusion;
pub mod derivation;
pub mod rules;

pub use conclusion::{
    KindingConclusion, NormalizingConclusion, SubtypeConclusion, TypingConclusion,
};
pub use derivation::{
    DefinitionDerivation, KindingDerivation, NormalizingDerivation, ProgramDerivation,
    SubtypeDerivation, TypingDerivation,
};
pub use rules::{SubtypeRule, TypingRule};

#[derive(Debug)]
pub enum Derivation<Lang>
where
    Lang: Language,
{
    TypingDerivation(TypingDerivation<Lang>),
    ProgramDerivation(ProgramDerivation<Lang>),
    DefinitionDerivation(DefinitionDerivation<Lang>),
    SubtypeDerivation(SubtypeDerivation<Lang>),
    NormalizingDerivation(NormalizingDerivation<Lang>),
    KindingDerivation(KindingDerivation<Lang>),
}

impl<Lang> Derivation<Lang>
where
    Lang: Language,
{
    pub fn ret_ty(&self) -> Lang::Type {
        match self {
            Self::DefinitionDerivation(deriv) => deriv.ret_ty(),
            Self::TypingDerivation(typ) => typ.ret_ty(),
            Self::SubtypeDerivation(sub) => sub.ret_ty(),
            Self::ProgramDerivation(deriv) => deriv.ret_ty(),
            Self::NormalizingDerivation(deriv) => deriv.ret_ty(),
            Self::KindingDerivation(deriv) => deriv.ret_ty(),
        }
    }

    /// Turn `Self` into [`DefinitionDerivation`]
    /// # Errors
    /// Returns an error if `Self` is a different derivation
    pub fn into_def(self) -> Result<DefinitionDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Definition Derivation";
        match self {
            Self::DefinitionDerivation(deriv) => Ok(deriv),
            Self::TypingDerivation(_) => Err(UnexpectedDerivation::new("Typing Derivation", exp)),
            Self::ProgramDerivation(_) => Err(UnexpectedDerivation::new("Program Derivation", exp)),
            Self::SubtypeDerivation(_) => Err(UnexpectedDerivation::new("Subtype Derivation", exp)),
            Self::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Self::KindingDerivation(_) => Err(UnexpectedDerivation::new("Kinding Derivation", exp)),
        }
    }

    /// Turn `Self` into [`TypingDerivation`]
    /// # Errors
    /// Returns an error if `Self` is a different derivation
    pub fn into_ty(self) -> Result<TypingDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Typing Derivation";
        match self {
            Self::TypingDerivation(deriv) => Ok(deriv),
            Self::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
            Self::ProgramDerivation(_) => Err(UnexpectedDerivation::new("Program Derivation", exp)),
            Self::SubtypeDerivation(_) => Err(UnexpectedDerivation::new("Subtype Derivation", exp)),
            Self::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Self::KindingDerivation(_) => Err(UnexpectedDerivation::new("Kinding Derivation", exp)),
        }
    }

    /// Turn `Self` into [`ProgramDerivation`]
    /// # Errors
    /// Returns an error if `Self` is a different derivation
    pub fn into_prog(self) -> Result<ProgramDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Program Derivation";
        match self {
            Self::ProgramDerivation(deriv) => Ok(deriv),
            Self::TypingDerivation(_) => Err(UnexpectedDerivation::new("Typing Derivation", exp)),
            Self::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
            Self::SubtypeDerivation(_) => Err(UnexpectedDerivation::new("Subtype Derivation", exp)),
            Self::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Self::KindingDerivation(_) => Err(UnexpectedDerivation::new("Kinding Derivation", exp)),
        }
    }

    /// Turn `Self` into [`KindingDerivation`]
    /// # Errors
    /// Returns an error if `Self` is a different derivation
    pub fn into_kind(self) -> Result<KindingDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Kinding Derivation";
        match self {
            Self::KindingDerivation(deriv) => Ok(deriv),
            Self::TypingDerivation(_) => Err(UnexpectedDerivation::new("Typing Derivation", exp)),
            Self::SubtypeDerivation(_) => Err(UnexpectedDerivation::new("Subtype Derivation", exp)),
            Self::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Self::ProgramDerivation(_) => Err(UnexpectedDerivation::new("Program Derivation", exp)),
            Self::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
        }
    }
}

impl<Lang> From<TypingDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: TypingDerivation<Lang>) -> Self {
        Self::TypingDerivation(deriv)
    }
}

impl<Lang> From<ProgramDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: ProgramDerivation<Lang>) -> Self {
        Self::ProgramDerivation(deriv)
    }
}

impl<Lang> From<DefinitionDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: DefinitionDerivation<Lang>) -> Self {
        Self::DefinitionDerivation(deriv)
    }
}

impl<Lang> fmt::Display for Derivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TypingDerivation(deriv) => deriv.fmt(f),
            Self::ProgramDerivation(deriv) => deriv.fmt(f),
            Self::DefinitionDerivation(deriv) => deriv.fmt(f),
            Self::SubtypeDerivation(deriv) => deriv.fmt(f),
            Self::NormalizingDerivation(deriv) => deriv.fmt(f),
            Self::KindingDerivation(deriv) => deriv.fmt(f),
        }
    }
}
