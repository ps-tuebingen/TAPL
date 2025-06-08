use super::super::LatexFmt;
use eval::values::{Cons, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Cons<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "Cons[{}]({},{})",
            self.ty.to_latex(),
            self.head.to_latex(),
            self.tail.to_latex()
        )
    }
}
