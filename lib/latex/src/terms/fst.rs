use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Fst};

impl<Lang> LatexFmt for Fst<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{}.\\text{{fst}}", self.term.to_latex(conf))
    }
}
