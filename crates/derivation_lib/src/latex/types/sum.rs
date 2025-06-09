use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Sum, Type};

impl<Ty> LatexFmt for Sum<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "({}+{})",
            self.left.to_latex(conf),
            self.right.to_latex(conf)
        )
    }
}
