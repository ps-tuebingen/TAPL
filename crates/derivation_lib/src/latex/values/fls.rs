use super::super::LatexFmt;
use eval::values::False;
use syntax::terms::Term;

impl<T> LatexFmt for False<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("False")
    }
}
