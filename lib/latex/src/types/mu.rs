use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Mu};

impl<Lang> LatexFmt for Mu<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\mu {}. {}",
            self.var.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
