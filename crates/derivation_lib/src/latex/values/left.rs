use super::super::LatexFmt;
use eval::values::{Left, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Left<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "inl({}) as {}",
            self.left_val.to_latex(),
            self.ty.to_latex()
        )
    }
}
