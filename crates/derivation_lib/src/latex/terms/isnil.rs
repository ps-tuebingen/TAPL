use super::super::LatexFmt;
use syntax::{
    terms::{IsNil, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for IsNil<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("isnil[{}]({})", self.ty.to_latex(), self.term.to_latex())
    }
}
