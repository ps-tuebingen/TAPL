use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Fix, Term};

impl<T> LatexFmt for Fix<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{fix}} ({})", self.term.to_latex(conf))
    }
}
