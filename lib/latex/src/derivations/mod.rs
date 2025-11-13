use super::{LatexConfig, LatexFmt};
use derivations::Derivation;
use syntax::language::Language;

mod definition;
mod kinding;
mod normalizing;
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
            Self::TypingDerivation(deriv) => deriv.to_latex(conf),
            Self::DefinitionDerivation(deriv) => deriv.to_latex(conf),
            Self::ProgramDerivation(deriv) => deriv.to_latex(conf),
            Self::SubtypeDerivation(deriv) => deriv.to_latex(conf),
            Self::NormalizingDerivation(deriv) => deriv.to_latex(conf),
            Self::KindingDerivation(deriv) => deriv.to_latex(conf),
        }
    }
}

pub fn buss_conc(conc_formatted: &str, num_premises: usize) -> String {
    match num_premises {
        0 | 1 => format!("\\UnaryInfC{{${conc_formatted}$}}",),
        2 => format!("\\BinaryInfC{{${conc_formatted}$}}",),
        3 => format!("\\TrinaryInfC{{${conc_formatted}$}}",),
        4 => format!("\\QuaternaryInfC{{${conc_formatted}$}}",),
        5 => format!("\\QuinaryInfC{{${conc_formatted}$}}",),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    }
}
