use super::super::LatexFmt;
use syntax::{
    terms::{Nil, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Nil<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("nil[{}]", self.ty.to_latex())
    }
}
