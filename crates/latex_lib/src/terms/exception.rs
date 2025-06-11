use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Exception<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{error}}[{}]", self.ty.to_latex(conf))
    }
}
