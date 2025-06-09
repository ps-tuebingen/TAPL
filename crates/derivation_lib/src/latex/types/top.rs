use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Top, Type};

impl<Ty> LatexFmt for Top<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Top}}[{}]", self.kind.to_latex(conf))
    }
}
