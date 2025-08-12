use super::super::{LatexConfig, LatexFmt};
use syntax::types::{OpApp, Type};

impl<Ty> LatexFmt for OpApp<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("({}[{}])", self.fun.to_latex(conf), self.arg.to_latex(conf))
    }
}
