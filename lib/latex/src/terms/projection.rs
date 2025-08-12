use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Projection, Term};

impl<T> LatexFmt for Projection<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{}.{}", self.term.to_latex(conf), self.index)
    }
}
