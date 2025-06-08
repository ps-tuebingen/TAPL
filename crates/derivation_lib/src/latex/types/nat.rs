use super::super::LatexFmt;
use syntax::types::{Nat, Type};

impl<Ty> LatexFmt for Nat<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("Nat")
    }
}
