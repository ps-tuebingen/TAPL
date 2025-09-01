use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Reference};

impl<Lang> LatexFmt for Reference<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Ref}}[{}]", self.ty.to_latex(conf))
    }
}
