use super::super::LatexFmt;
use syntax::terms::{Projection, Term};

impl<T> LatexFmt for Projection<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}.{}", self.term.to_latex(), self.index)
    }
}
