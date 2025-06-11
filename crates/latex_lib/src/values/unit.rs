use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::Unit};

impl<T> LatexFmt for Unit<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "unit".to_owned()
    }
}
