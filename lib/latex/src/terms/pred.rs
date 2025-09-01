use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Pred};

impl<Lang> LatexFmt for Pred<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{pred}}({})", self.term.to_latex(conf))
    }
}
