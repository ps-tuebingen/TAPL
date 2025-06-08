use syntax::terms::{Term,False};
use super::super::LatexFmt;

impl<T> LatexFmt for False<T> where T:Term+LatexFmt{
    fn to_latex(&self) -> String{
        format!("false")
    }
}
