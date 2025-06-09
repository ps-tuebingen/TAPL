use super::super::{LatexConfig, LatexFmt};
use eval::values::{Right, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Right<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "inl({}) as {}",
            self.right_val.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
