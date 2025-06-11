use super::super::{LatexConfig, LatexFmt};
use syntax::types::{ForallBounded, Type};

impl<Ty> LatexFmt for ForallBounded<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\forall {}<:{}.{}",
            self.var,
            self.sup_ty.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
