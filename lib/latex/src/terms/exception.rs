use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Exception};

impl<Lang> LatexFmt for Exception<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{error}}[{}]", self.ty.to_latex(conf))
    }
}
