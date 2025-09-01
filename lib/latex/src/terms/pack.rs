use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Pack};

impl<Lang> LatexFmt for Pack<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{*({}),{}\\}} \\text{{as}} {}",
            self.inner_ty.to_latex(conf),
            self.term.to_latex(conf),
            self.outer_ty.to_latex(conf)
        )
    }
}
