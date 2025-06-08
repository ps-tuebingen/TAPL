use super::super::LatexFmt;
use syntax::terms::{Term, Variable};

impl<T> LatexFmt for Variable<T>
where
    T: Term,
{
    fn to_latex(&self) -> String {
        format!("{}", self.var)
    }
}
