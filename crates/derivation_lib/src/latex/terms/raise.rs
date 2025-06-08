use super::super::LatexFmt;
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Raise<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "raise[{}]({} : {})",
            self.cont_ty.to_latex(),
            self.exception.to_latex(),
            self.exception_ty.to_latex()
        )
    }
}
