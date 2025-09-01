use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::List};

impl<Lang> LatexFmt for List<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{List}}[{}]", self.ty.to_latex(conf))
    }
}
