use super::super::{LatexConfig, LatexFmt};
use syntax::{
    types::Type,
    values::{Value, Variant},
};

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
