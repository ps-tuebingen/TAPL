use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::False};

impl<Lang> LatexFmt for False<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{ false }".to_owned()
    }
}
