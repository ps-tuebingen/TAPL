use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Unit};

impl<Lang> LatexFmt for Unit<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{Unit}".to_owned()
    }
}
