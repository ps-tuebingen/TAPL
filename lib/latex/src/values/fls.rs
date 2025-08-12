use super::super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::False};

impl<T> LatexFmt for False<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "\\text{ false }".to_string()
    }
}
