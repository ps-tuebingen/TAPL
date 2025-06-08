use super::super::LatexFmt;
use syntax::types::{ForallBounded, Type};

impl<Ty> LatexFmt for ForallBounded<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\forall {}<:{}.{}",
            self.var,
            self.sup_ty.to_latex(),
            self.ty.to_latex()
        )
    }
}
