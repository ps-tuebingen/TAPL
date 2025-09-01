use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::TyLambda};

impl<Lang> LatexFmt for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}::{}.{}",
            self.var.to_latex(conf),
            self.annot.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
