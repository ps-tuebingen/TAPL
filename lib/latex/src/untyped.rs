use super::{LatexConfig, LatexFmt};
use syntax::{language::Language, untyped::Untyped};

impl<Lang> LatexFmt for Untyped<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        String::new()
    }
}
