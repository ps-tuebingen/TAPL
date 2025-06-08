use super::super::LatexFmt;
use syntax::terms::{IsZero, Term};

impl<T> LatexFmt for IsZero<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("iszero({})", self.term)
    }
}
