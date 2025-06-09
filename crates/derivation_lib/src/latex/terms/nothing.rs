use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Nothing<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{Nothing}}[{}]", self.ty.to_latex(conf))
    }
}
