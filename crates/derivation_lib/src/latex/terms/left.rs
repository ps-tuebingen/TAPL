use super::super::LatexFmt;
use syntax::{
    terms::{Left, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Left<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "inl({}) as {}",
            self.left_term.to_latex(),
            self.ty.to_latex()
        )
    }
}
