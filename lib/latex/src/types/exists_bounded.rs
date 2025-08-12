use super::super::{LatexConfig, LatexFmt};
use syntax::types::{ExistsBounded, Type};

impl<Ty> LatexFmt for ExistsBounded<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{\\exists {} <: {},{}\\}}",
            self.var.to_latex(conf),
            self.sup_ty.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
