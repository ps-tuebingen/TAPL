use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::OpLambda};

impl<Lang> LatexFmt for OpLambda<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}::{}.{}",
            self.var.to_latex(conf),
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
