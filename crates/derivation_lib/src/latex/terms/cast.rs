use super::super::LatexFmt;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Cast<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{} as {}", self.term, self.ty)
    }
}
