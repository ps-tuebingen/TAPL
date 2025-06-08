use super::super::LatexFmt;
use eval::values::{Pack, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Pack<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{*{},{}\\}} as {}",
            self.inner_ty.to_latex(),
            self.val.to_latex(),
            self.outer_ty.to_latex()
        )
    }
}
