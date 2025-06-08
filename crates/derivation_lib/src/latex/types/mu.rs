use super::super::LatexFmt;
use syntax::types::{Mu, Type};

impl<Ty> LatexFmt for Mu<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("\\mu {}. {}", self.var, self.ty.to_latex())
    }
}
