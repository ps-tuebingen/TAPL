use super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, untyped::Untyped};

impl<T> LatexFmt for Untyped<T>
where
    T: Term,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        "".to_owned()
    }
}
