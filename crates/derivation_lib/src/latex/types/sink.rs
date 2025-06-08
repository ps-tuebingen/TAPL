use super::super::LatexFmt;
use syntax::types::{Sink, Type};

impl<Ty> LatexFmt for Sink<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Sink[{}]", self.ty.to_latex())
    }
}
