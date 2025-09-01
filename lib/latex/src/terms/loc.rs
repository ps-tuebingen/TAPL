use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Loc};

impl<Lang> LatexFmt for Loc<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.loc)
    }
}
