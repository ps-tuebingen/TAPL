use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Nat};

impl<Lang> LatexFmt for Nat<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{Nat}".to_owned()
    }
}
