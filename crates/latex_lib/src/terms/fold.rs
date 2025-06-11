use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Fold, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Fold<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{fold}}[{}]{}",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
