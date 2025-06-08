use super::super::LatexFmt;
use syntax::types::{Top, Type};

impl<Ty> LatexFmt for Top<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Top[{}]", self.kind.to_latex())
    }
}
