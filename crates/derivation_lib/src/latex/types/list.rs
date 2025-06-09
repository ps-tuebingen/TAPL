use super::super::{LatexConfig, LatexFmt};
use syntax::types::{List, Type};

impl<Ty> LatexFmt for List<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("List[{}]", self.ty.to_latex(conf))
    }
}
