use super::super::LatexFmt;
use syntax::{
    terms::{Tail, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Tail<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("tail[{}]({})", self.ty.to_latex(), self.term.to_latex())
    }
}
