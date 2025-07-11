use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Deref, Term};

impl<T> LatexFmt for Deref<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("!{}", self.term.to_latex(conf))
    }
}
