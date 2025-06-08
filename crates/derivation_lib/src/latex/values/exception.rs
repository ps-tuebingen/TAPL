use super::super::LatexFmt;
use eval::values::Exception;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Exception<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("error[{}]", self.ty.to_latex())
    }
}
