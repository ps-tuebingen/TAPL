use super::super::LatexFmt;
use eval::values::Nil;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Nil<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Nil[{}]", self.ty.to_latex())
    }
}
