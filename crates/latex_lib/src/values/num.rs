use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::Num};

impl<T> LatexFmt for Num<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.num)
    }
}
