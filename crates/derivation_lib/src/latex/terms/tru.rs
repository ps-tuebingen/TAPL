use super::super::LatexFmt;
use syntax::terms::{Term, True};

impl<T> LatexFmt for True<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("true")
    }
}
