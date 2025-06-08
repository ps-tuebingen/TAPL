use super::super::LatexFmt;
use eval::values::LambdaSub;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for LambdaSub<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\lambda {} <:({}).{}",
            self.var,
            self.sup_ty.to_latex(),
            self.term.to_latex()
        )
    }
}
