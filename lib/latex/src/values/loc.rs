use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Loc};

impl<Lang> LatexFmt for Loc<Lang>
where
    Lang: Language,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.loc)
    }
}
