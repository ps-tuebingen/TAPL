use super::super::LatexFmt;
use eval::values::{Fold, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Fold<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("fold[{}]({})", self.ty.to_latex(), self.val.to_latex())
    }
}
