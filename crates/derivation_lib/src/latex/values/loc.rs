use super::super::LatexFmt;
use eval::values::Loc;
use syntax::terms::Term;

impl<T> LatexFmt for Loc<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}", self.loc)
    }
}
