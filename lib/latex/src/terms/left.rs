use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Left};

impl<Lang> LatexFmt for Left<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{inl}}({}) \\text{{as}} {}",
            self.left_term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
