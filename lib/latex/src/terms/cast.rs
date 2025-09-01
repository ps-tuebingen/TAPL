use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Cast};

impl<Lang> LatexFmt for Cast<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{}:{}", self.term.to_latex(conf), self.ty.to_latex(conf))
    }
}
