use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Raise};

impl<Lang> LatexFmt for Raise<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{raise}}[{},{}]({})",
            self.cont_ty.to_latex(conf),
            self.exception_ty.to_latex(conf),
            self.exception.to_latex(conf),
        )
    }
}
