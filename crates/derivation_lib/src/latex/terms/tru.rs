use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, True};

impl<T> LatexFmt for True<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("true")
    }
}
