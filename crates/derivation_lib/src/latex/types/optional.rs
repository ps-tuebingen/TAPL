use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Optional, Type};

impl<Ty> LatexFmt for Optional<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Optional[{}]", self.ty.to_latex(conf))
    }
}
