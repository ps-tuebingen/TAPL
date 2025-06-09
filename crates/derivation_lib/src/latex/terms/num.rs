use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Num, Term};

impl<T> LatexFmt for Num<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.num)
    }
}
