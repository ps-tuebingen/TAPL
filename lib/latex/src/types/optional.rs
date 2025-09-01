use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Optional};

impl<Lang> LatexFmt for Optional<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Optional}}[{}]", self.ty.to_latex(conf))
    }
}
