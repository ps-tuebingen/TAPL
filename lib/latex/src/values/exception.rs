use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Exception};

impl<Lang> LatexFmt for Exception<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("error[{}]", self.ty.to_latex(conf))
    }
}
