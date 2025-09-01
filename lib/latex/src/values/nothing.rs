use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Nothing};

impl<Lang> LatexFmt for Nothing<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Nothing[{}]", self.ty.to_latex(conf))
    }
}
