use super::super::LatexFmt;
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Exception<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("error[{}]", self.ty.to_latex())
    }
}
