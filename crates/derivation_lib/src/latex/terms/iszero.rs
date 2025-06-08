use super::super::LatexFmt;
use syntax::terms::{IsZero, Term};

impl<T> LatexFmt for IsZero<T> {
    fn to_latex(&self) -> String {
        format!("iszero({})", self.term)
    }
}
