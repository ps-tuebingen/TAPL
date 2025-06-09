use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Something, Term};

impl<T> LatexFmt for Something<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("something({})", self.term.to_latex(conf))
    }
}
