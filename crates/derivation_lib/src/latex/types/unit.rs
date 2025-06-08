use super::super::LatexFmt;
use syntax::types::{Type, Unit};

impl<Ty> LatexFmt for Unit<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Unit")
    }
}
