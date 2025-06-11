use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Pack<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{*({}),{}\\}} \\text{{as}} {}",
            self.inner_ty.to_latex(conf),
            self.term.to_latex(conf),
            self.outer_ty.to_latex(conf)
        )
    }
}
