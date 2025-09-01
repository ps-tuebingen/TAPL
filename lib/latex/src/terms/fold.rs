use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Fold};

impl<Lang> LatexFmt for Fold<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{fold}}[{}]{}",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
