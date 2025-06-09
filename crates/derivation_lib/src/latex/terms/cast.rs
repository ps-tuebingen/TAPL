use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Cast<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "{} \\text{{as}} {}",
            self.term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
