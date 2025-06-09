use super::super::{LatexConfig, LatexFmt};
use eval::values::{Value, Variant};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Variant<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} = {} \\rangle",
            self.label,
            self.val.to_latex(conf)
        )
    }
}
