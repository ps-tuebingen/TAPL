use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, TyLambda};

impl<T> LatexFmt for TyLambda<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}::{}.({})",
            self.var,
            self.annot.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
