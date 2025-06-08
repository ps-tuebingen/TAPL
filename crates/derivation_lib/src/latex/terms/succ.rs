use super::super::LatexFmt;
use syntax::terms::{Succ, Term};

impl<T> LatexFmt for Succ<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("succ({})", self.term.to_latex())
    }
}
