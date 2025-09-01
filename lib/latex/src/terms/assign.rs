use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Assign};

impl<Lang> LatexFmt for Assign<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{} := {}", self.lhs.to_latex(conf), self.rhs.to_latex(conf))
    }
}
