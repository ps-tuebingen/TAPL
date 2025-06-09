use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Term, Unfold},
    types::Type,
};

impl<T, Ty> LatexFmt for Unfold<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "unfold[{}]({})",
            self.ty.to_latex(conf),
            self.term.to_latex(conf)
        )
    }
}
