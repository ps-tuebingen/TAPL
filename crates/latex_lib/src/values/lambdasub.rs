use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, types::Type, values::LambdaSub};

impl<T, Ty> LatexFmt for LambdaSub<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {} <:({}).{}",
            self.var,
            self.sup_ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
