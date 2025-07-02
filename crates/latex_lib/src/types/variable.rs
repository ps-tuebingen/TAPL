use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Type, TypeVariable};

impl<Ty> LatexFmt for TypeVariable<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _conf: &mut LatexConfig) -> String {
        format!("\\text{{{}}}", self.v.to_latex(_conf))
    }
}
