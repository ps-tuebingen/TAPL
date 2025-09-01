use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::LambdaSub};

impl<Lang> LatexFmt for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}<:{}.{}",
            self.var.to_latex(conf),
            self.sup_ty.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
