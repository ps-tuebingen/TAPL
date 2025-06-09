use super::super::{LatexConfig, LatexFmt};
use eval::values::Nothing;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Nothing<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Nothing[{}]", self.ty.to_latex(conf))
    }
}
