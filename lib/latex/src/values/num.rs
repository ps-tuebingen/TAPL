use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Num};

impl<Lang> LatexFmt for Num<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.num)
    }
}
