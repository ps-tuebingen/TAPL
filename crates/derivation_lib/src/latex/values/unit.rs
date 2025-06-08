use super::super::LatexFmt;
use eval::values::Unit;
use syntax::terms::Term;

impl<T> LatexFmt for Unit<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("unit")
    }
}
