use super::{LatexConfig, LatexFmt};
use derivations::Derivation;
use syntax::language::Language;

mod definition;
mod program;
mod subtyping;
mod typing;

impl<Lang> LatexFmt for Derivation<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
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
