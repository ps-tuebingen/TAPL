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
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{} as {}", self.term, self.ty)
    }
}
