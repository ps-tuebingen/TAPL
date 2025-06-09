use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Succ, Term};

impl<T> LatexFmt for Succ<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("succ({})", self.term.to_latex(conf))
    }
}
