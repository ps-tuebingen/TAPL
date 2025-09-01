use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Source};

impl<Lang> LatexFmt for Source<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Source}}[{}]", self.ty.to_latex(conf))
    }
}
