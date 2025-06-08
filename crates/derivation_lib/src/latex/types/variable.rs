use super::super::LatexFmt;
use syntax::types::{Type, TypeVariable};

impl<Ty> LatexFmt for TypeVariable<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{}", self.v)
    }
}
