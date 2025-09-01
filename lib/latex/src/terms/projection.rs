use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Projection};

impl<Lang> LatexFmt for Projection<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{}.{}", self.term.to_latex(conf), self.index)
    }
}
