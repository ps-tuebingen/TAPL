use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Forall, Type};

impl<Ty> LatexFmt for Forall<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\forall {} :: {}. {}",
            self.var,
            self.kind.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
