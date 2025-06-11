use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{RecordProj, Term};

impl<T> LatexFmt for RecordProj<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("({}).{}", self.record.to_latex(conf), self.label)
    }
}
