use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{IsZero, Term};

impl<T> LatexFmt for IsZero<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{iszero({})}}", self.term.to_latex(conf))
    }
}
