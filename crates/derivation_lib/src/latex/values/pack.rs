use super::super::{LatexConfig, LatexFmt};
use eval::values::{Pack, Value};
use syntax::types::Type;

impl<V, Ty> LatexFmt for Pack<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{*{},{}\\}} as {}",
            self.inner_ty.to_latex(conf),
            self.val.to_latex(conf),
            self.outer_ty.to_latex(conf)
        )
    }
}
