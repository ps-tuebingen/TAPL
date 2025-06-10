use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, types::Type, values::Nil};

impl<T, Ty> LatexFmt for Nil<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("Nil[{}]", self.ty.to_latex(conf))
    }
}
