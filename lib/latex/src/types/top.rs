use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Top};

impl<Lang> LatexFmt for Top<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Top}}[{}]", self.kind.to_latex(conf))
    }
}
