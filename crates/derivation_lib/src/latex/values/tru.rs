use super::super::LatexFmt;
use eval::values::True;
use syntax::terms::Term;

impl<T> LatexFmt for True<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("true")
    }
}
