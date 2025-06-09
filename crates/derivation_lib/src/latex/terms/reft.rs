use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Ref, Term};

impl<T> LatexFmt for Ref<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("\\text{{ref}}({})", self.term.to_latex(conf))
    }
}
