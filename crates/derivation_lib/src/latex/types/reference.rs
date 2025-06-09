use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Reference, Type};

impl<Ty> LatexFmt for Reference<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Ref[{}]", self.ty.to_latex(conf))
    }
}
