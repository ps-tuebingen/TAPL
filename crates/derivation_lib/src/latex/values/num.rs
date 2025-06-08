use super::super::LatexFmt;
use eval::values::Num;
use syntax::terms::Term;

impl<T> LatexFmt for Num<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}", self.num)
    }
}
