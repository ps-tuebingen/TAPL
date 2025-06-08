use syntax::{types::Type,terms::{Head,Term}};
use super::super::LatexFmt;

impl<T,Ty> LatexFmt for Head<T,Ty> where T:Term+LatexFmt,Ty:Type+LatexFmt{
    fn to_latex(&self) -> String{
        format!("head[{}]({})",self.ty.to_latex(),self.term.to_latex())
    }
}
