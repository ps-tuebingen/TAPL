use super::{LatexConfig, LatexFmt};
use derivations::Derivation;
use syntax::{terms::Term, types::Type};

mod definition;
mod program;
mod subtyping;
mod typing;

impl<T, Ty> LatexFmt for Derivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Derivation::TypingDerivation(deriv) => deriv.to_latex(conf),
            Derivation::DefinitionDerivation(deriv) => deriv.to_latex(conf),
            Derivation::ProgramDerivation(deriv) => deriv.to_latex(conf),
            Derivation::SubtypeDerivation(deriv) => deriv.to_latex(conf),
        }
    }
}
