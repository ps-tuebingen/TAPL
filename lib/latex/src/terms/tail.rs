use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Tail};

impl<Lang> LatexFmt for Tail<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{tail}}[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
