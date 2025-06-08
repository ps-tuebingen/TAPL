use super::super::LatexFmt;
use syntax::{
    terms::{Term, Unfold},
    types::Type,
};

impl<T, Ty> LatexFmt for Unfold<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("unfold[{}]({})", self.ty.to_latex(), self.term.to_latex())
    }
}
