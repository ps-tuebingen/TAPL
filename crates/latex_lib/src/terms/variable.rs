use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, Variable};

impl<T> LatexFmt for Variable<T>
where
    T: Term,
{
    fn to_latex(&self, _conf: &mut LatexConfig) -> String {
        format!("\\text{{ {} }}", self.var.to_latex(_conf))
    }
}
