use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Ascribe<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{} : {}", self.term.to_latex(conf), self.ty.to_latex(conf))
    }
}
