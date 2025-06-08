use super::super::LatexFmt;
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Pack<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{*({}),{}\\}} as {}",
            self.inner_ty.to_latex(),
            self.term.to_latex(),
            self.outer_ty.to_latex()
        )
    }
}
