use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::True};

impl<Lang> LatexFmt for True<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{ true }".to_owned()
    }
}
