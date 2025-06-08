use super::super::LatexFmt;
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Right<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "inl({}) as {}",
            self.right_term.to_latex(),
            self.ty.to_latex()
        )
    }
}
