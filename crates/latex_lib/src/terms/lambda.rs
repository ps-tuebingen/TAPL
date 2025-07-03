use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Lambda, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Lambda<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}:{}.{}",
            self.var.to_latex(conf),
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
