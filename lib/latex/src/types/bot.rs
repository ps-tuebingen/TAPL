use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Bot, Type};

impl<Ty> LatexFmt for Bot<Ty>
where
    Ty: Type,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{Bot}".to_owned()
    }
}
