use super::super::LatexFmt;
use syntax::terms::{Fix, Term};

impl<T> LatexFmt for Fix<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("fix ({})", self.term.to_latex())
    }
}
