use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Variant};

impl<Lang> LatexFmt for Variant<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} = {} \\rangle \\text{{as}} {}",
            self.label,
            self.term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
