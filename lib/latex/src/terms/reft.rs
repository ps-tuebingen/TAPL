use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Ref};

impl<Lang> LatexFmt for Ref<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{ref}}({})", self.term.to_latex(conf))
    }
}
