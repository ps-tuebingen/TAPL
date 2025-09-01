use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Bool};

impl<Lang> LatexFmt for Bool<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{Bool}".to_owned()
    }
}
