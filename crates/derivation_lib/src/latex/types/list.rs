use super::super::LatexFmt;
use syntax::types::{List, Type};

impl<Ty> LatexFmt for List<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("List[{}]", self.ty.to_latex())
    }
}
