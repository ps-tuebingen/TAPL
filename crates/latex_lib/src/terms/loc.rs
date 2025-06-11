use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Loc, Term};

impl<T> LatexFmt for Loc<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("{}", self.loc)
    }
}
