use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Nothing};

impl<Lang> LatexFmt for Nothing<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Nothing}}[{}]", self.ty.to_latex(conf))
    }
}
