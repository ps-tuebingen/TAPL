use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Unit};

impl<Lang> LatexFmt for Unit<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{{unit}}".to_owned()
    }
}
