use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Sink, Type};

impl<Ty> LatexFmt for Sink<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Sink}}[{}]", self.ty.to_latex(conf))
    }
}
