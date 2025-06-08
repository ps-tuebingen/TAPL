use super::super::LatexFmt;
use syntax::{
    terms::{Fold, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Fold<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("fold[{}]{}", self.ty.to_latex(), self.term.to_latex())
    }
}
