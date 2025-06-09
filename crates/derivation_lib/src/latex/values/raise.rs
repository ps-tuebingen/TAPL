use super::super::{LatexConfig, LatexFmt};
use eval::values::{Raise, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Raise<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "raise[{}]({} : {})",
            self.cont_ty.to_latex(conf),
            self.val.to_latex(conf),
            self.cont_ty.to_latex(conf)
        )
    }
}
