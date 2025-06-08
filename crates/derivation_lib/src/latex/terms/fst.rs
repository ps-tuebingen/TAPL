use super::super::LatexFmt;
use syntax::terms::{Fst, Term};

impl<T> LatexFmt for Fst<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}.0", self.term.to_latex())
    }
}
