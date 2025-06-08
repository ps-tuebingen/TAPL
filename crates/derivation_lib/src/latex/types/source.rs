use super::super::LatexFmt;
use syntax::types::{Source, Type};

impl<Ty> LatexFmt for Source<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Source[{}]", self.ty.to_latex())
    }
}
