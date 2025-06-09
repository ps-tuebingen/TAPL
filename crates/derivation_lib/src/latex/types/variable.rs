use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Type, TypeVariable};

impl<Ty> LatexFmt for TypeVariable<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        self.v.clone()
    }
}
