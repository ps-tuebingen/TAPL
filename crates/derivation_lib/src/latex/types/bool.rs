use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Bool, Type};

impl<Ty> LatexFmt for Bool<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("Bool")
    }
}
