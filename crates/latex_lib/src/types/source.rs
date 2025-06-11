use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Source, Type};

impl<Ty> LatexFmt for Source<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Source}}[{}]", self.ty.to_latex(conf))
    }
}
