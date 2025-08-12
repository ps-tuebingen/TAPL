use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Raise<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{raise}}[{},{}]({})",
            self.cont_ty.to_latex(conf),
            self.exception_ty.to_latex(conf),
            self.exception.to_latex(conf),
        )
    }
}
