use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, Variable};

impl<T> LatexFmt for Variable<T>
where
    T: Term,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("\\text{{ {} }}", self.var.replace("_", "\\_"))
    }
}
