use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::LambdaSub};

impl<Lang> LatexFmt for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {} <:({}).{}",
            self.var.to_latex(conf),
            self.sup_ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
