use super::super::LatexFmt;
use syntax::terms::{Term,Assign};

impl<T> LatexFmt for Assign<T> where T:Term+LatexFmt{
    fn to_latex(&self) -> String {
        format!("{} := {}",self.lhs.to_latex(),self.rhs.to_latex())

    }
}
