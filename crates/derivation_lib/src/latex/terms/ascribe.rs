use super::super::LatexFmt;
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Ascribe<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{} as {}", self.term.to_latex(), self.ty.to_latex())
    }
}
