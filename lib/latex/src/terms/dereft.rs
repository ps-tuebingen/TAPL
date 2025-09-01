use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Deref};

impl<Lang> LatexFmt for Deref<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("!{}", self.term.to_latex(conf))
    }
}
