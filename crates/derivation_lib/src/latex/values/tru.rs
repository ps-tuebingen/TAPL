use super::super::{LatexConfig, LatexFmt};
use eval::values::True;
use syntax::terms::Term;

impl<T> LatexFmt for True<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("true")
    }
}
