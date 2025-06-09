use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Nat, Type};

impl<Ty> LatexFmt for Nat<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("Nat")
    }
}
