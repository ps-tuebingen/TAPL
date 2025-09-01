use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Right};

impl<Lang> LatexFmt for Right<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{inl}}({}) \\text{{as}} {}",
            self.right_term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
