use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::UntypedLambda};

impl<T> LatexFmt for UntypedLambda<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\lambda {}.{}", self.var, self.body.to_latex(conf))
    }
}
