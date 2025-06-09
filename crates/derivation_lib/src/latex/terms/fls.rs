use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{False, Term};

impl<T> LatexFmt for False<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "false".to_owned()
    }
}
