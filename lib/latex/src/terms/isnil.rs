use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::IsNil};

impl<Lang> LatexFmt for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{isnil}}[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
