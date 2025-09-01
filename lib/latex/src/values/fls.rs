use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::False};

impl<Lang> LatexFmt for False<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{ false }".to_string()
    }
}
