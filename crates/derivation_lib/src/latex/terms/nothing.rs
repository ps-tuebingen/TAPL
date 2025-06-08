use syntax::{types::Type,terms::{Term,Nothing}};
use super::super::LatexFmt;

impl<T,Ty> LatexFmt for Nothing<T,Ty> where T:Term+LatexFmt,Ty:Type+LatexFmt{
    fn to_latex(&self) -> String{
        format!("Nothing[{}]",self.ty.to_latex())
    }
}
