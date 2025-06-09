use super::super::{LatexConfig, LatexFmt};
use eval::values::Unit;
use syntax::terms::Term;

impl<T> LatexFmt for Unit<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "unit".to_owned()
    }
}
