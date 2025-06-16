use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Type, TypeVariable};

impl<Ty> LatexFmt for TypeVariable<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("\\text{{{}}}", self.v.replace("_", "\\_"))
    }
}
