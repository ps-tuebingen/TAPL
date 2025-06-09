use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Assign, Term};

impl<T> LatexFmt for Assign<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{} := {}", self.lhs.to_latex(conf), self.rhs.to_latex(conf))
    }
}
