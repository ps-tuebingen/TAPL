use super::super::LatexFmt;
use syntax::types::{Optional, Type};

impl<Ty> LatexFmt for Optional<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Optional[{}]", self.ty.to_latex())
    }
}
