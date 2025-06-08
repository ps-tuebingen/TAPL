use super::super::LatexFmt;
use eval::values::{Raise, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Raise<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "raise[{}]({} : {})",
            self.cont_ty.to_latex(),
            self.val.to_latex(),
            self.cont_ty.to_latex()
        )
    }
}
