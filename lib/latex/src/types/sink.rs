use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Sink};

impl<Lang> LatexFmt for Sink<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Sink}}[{}]", self.ty.to_latex(conf))
    }
}
