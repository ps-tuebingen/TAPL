use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Nil};

impl<Lang> LatexFmt for Nil<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Nil[{}]", self.ty.to_latex(conf))
    }
}
