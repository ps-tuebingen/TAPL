use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Num};

impl<Lang> LatexFmt for Num<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.num)
    }
}
