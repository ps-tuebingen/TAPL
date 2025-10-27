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
            Derivation::DefinitionDerivation(deriv) => deriv.ret_ty(),
            Derivation::TypingDerivation(typ) => typ.ret_ty(),
            Derivation::SubtypeDerivation(sub) => sub.ret_ty(),
            Derivation::ProgramDerivation(deriv) => deriv.ret_ty(),
            Derivation::NormalizingDerivation(deriv) => deriv.ret_ty(),
            Derivation::KindingDerivation(deriv) => deriv.ret_ty(),
        }
    }

    pub fn into_def(self) -> Result<DefinitionDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Definition Derivation";
        match self {
            Derivation::DefinitionDerivation(deriv) => Ok(deriv),
            Derivation::TypingDerivation(_) => {
                Err(UnexpectedDerivation::new("Typing Derivation", exp))
            }
            Derivation::ProgramDerivation(_) => {
                Err(UnexpectedDerivation::new("Program Derivation", exp))
            }
            Derivation::SubtypeDerivation(_) => {
                Err(UnexpectedDerivation::new("Subtype Derivation", exp))
            }
            Derivation::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Derivation::KindingDerivation(_) => {
                Err(UnexpectedDerivation::new("Kinding Derivation", exp))
            }
        }
    }

    pub fn into_ty(self) -> Result<TypingDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Typing Derivation";
        match self {
            Derivation::TypingDerivation(deriv) => Ok(deriv),
            Derivation::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
            Derivation::ProgramDerivation(_) => {
                Err(UnexpectedDerivation::new("Program Derivation", exp))
            }
            Derivation::SubtypeDerivation(_) => {
                Err(UnexpectedDerivation::new("Subtype Derivation", exp))
            }
            Derivation::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Derivation::KindingDerivation(_) => {
                Err(UnexpectedDerivation::new("Kinding Derivation", exp))
            }
        }
    }

    pub fn into_prog(self) -> Result<ProgramDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Program Derivation";
        match self {
            Derivation::ProgramDerivation(deriv) => Ok(deriv),
            Derivation::TypingDerivation(_) => {
                Err(UnexpectedDerivation::new("Typing Derivation", exp))
            }
            Derivation::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
            Derivation::SubtypeDerivation(_) => {
                Err(UnexpectedDerivation::new("Subtype Derivation", exp))
            }
            Derivation::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Derivation::KindingDerivation(_) => {
                Err(UnexpectedDerivation::new("Kinding Derivation", exp))
            }
        }
    }

    pub fn into_kind(self) -> Result<KindingDerivation<Lang>, UnexpectedDerivation> {
        let exp = "Kinding Derivation";
        match self {
            Derivation::KindingDerivation(deriv) => Ok(deriv),
            Derivation::TypingDerivation(_) => {
                Err(UnexpectedDerivation::new("Typing Derivation", exp))
            }
            Derivation::SubtypeDerivation(_) => {
                Err(UnexpectedDerivation::new("Subtype Derivation", exp))
            }
            Derivation::NormalizingDerivation(_) => {
                Err(UnexpectedDerivation::new("Normalizing Derivation", exp))
            }
            Derivation::ProgramDerivation(_) => {
                Err(UnexpectedDerivation::new("Program Derivation", exp))
            }
            Derivation::DefinitionDerivation(_) => {
                Err(UnexpectedDerivation::new("Definition Derivation", exp))
            }
        }
    }
}

impl<Lang> From<TypingDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: TypingDerivation<Lang>) -> Derivation<Lang> {
        Derivation::TypingDerivation(deriv)
    }
}

impl<Lang> From<ProgramDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: ProgramDerivation<Lang>) -> Derivation<Lang> {
        Derivation::ProgramDerivation(deriv)
    }
}

impl<Lang> From<DefinitionDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: DefinitionDerivation<Lang>) -> Derivation<Lang> {
        Derivation::DefinitionDerivation(deriv)
    }
}

impl<Lang> fmt::Display for Derivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Derivation::TypingDerivation(deriv) => deriv.fmt(f),
            Derivation::ProgramDerivation(deriv) => deriv.fmt(f),
            Derivation::DefinitionDerivation(deriv) => deriv.fmt(f),
            Derivation::SubtypeDerivation(deriv) => deriv.fmt(f),
            Derivation::NormalizingDerivation(deriv) => deriv.fmt(f),
            Derivation::KindingDerivation(deriv) => deriv.fmt(f),
        }
    }
}
