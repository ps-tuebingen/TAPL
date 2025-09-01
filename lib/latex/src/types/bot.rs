use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Bot};

impl<Lang> LatexFmt for Bot<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{Bot}".to_owned()
    }
}
