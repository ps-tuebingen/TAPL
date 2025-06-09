use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Tail, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Tail<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{tail}}[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
