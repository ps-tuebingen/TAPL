use super::super::LatexFmt;
use syntax::types::{Bool, Type};

impl<Ty> LatexFmt for Bool<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Bool")
    }
}
