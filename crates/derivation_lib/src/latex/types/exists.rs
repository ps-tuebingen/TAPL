use super::super::LatexFmt;
use syntax::types::{Exists, Type};

impl<Ty> LatexFmt for Exists<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{\\exists {} :: {}, {} \\}}",
            self.var,
            self.kind.to_latex(),
            self.ty.to_latex()
        )
    }
}
