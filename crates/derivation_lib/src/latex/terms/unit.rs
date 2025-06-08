use super::super::LatexFmt;
use syntax::terms::{Term, Unit};

impl<T> LatexFmt for Unit<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("unit")
    }
}
