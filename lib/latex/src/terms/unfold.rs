use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Unfold};

impl<Lang> LatexFmt for Unfold<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{unfold}}[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
