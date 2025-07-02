use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::TyLambda};

impl<T> LatexFmt for TyLambda<T>
where
    T: Term + LatexFmt,
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
