use super::super::LatexFmt;
use syntax::types::{Forall, Type};

impl<Ty> LatexFmt for Forall<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\forall {} :: {}. {}",
            self.var,
            self.kind.to_latex(),
            self.ty.to_latex()
        )
    }
}
