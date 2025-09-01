use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Forall};

impl<Lang> LatexFmt for Forall<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\forall {} :: {}. {}",
            self.var.to_latex(conf),
            self.kind.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
