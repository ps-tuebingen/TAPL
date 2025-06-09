use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Type, Unit};

impl<Ty> LatexFmt for Unit<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "Unit".to_owned()
    }
}
