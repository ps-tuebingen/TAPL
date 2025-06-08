use super::super::LatexFmt;
use syntax::terms::{Num, Term};

impl<T> LatexFmt for Num<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}", self.num)
    }
}
