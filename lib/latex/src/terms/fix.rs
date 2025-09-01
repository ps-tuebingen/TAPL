use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Fix};

impl<Lang> LatexFmt for Fix<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{fix}} ({})", self.term.to_latex(conf))
    }
}
