use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, types::Type, values::Exception};

impl<T, Ty> LatexFmt for Exception<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("error[{}]", self.ty.to_latex(conf))
    }
}
