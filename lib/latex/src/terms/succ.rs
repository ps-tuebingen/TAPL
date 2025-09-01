use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Succ};

impl<Lang> LatexFmt for Succ<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{succ}}({})", self.term.to_latex(conf))
    }
}
