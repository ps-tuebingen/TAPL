use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> LatexFmt for OpLambdaSub<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}<:{}.{}",
            self.var.to_latex(conf),
            self.sup.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
