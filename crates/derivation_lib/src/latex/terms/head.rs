use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Head, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Head<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{head}}[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
