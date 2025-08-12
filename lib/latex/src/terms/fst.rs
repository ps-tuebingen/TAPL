use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Fst, Term};

impl<T> LatexFmt for Fst<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{}.\\text{{fst}}", self.term.to_latex(conf))
    }
}
