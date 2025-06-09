use super::super::{LatexConfig, LatexFmt};
use eval::values::Lambda;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Lambda<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}:{}.{}",
            self.var,
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
