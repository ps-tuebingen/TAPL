use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Nil};

impl<Lang> LatexFmt for Nil<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{nil}}[{}]", self.ty.to_latex(conf))
    }
}
