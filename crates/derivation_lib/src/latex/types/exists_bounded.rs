use super::super::LatexFmt;
use syntax::types::{ExistsBounded, Type};

impl<Ty> LatexFmt for ExistsBounded<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{\\exists {} <: {},{}\\}}",
            self.var,
            self.sup_ty.to_latex(),
            self.ty.to_latex()
        )
    }
}
