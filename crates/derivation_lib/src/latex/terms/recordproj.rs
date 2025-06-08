use super::super::LatexFmt;
use syntax::terms::{RecordProj, Term};

impl<T> LatexFmt for RecordProj<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("({}).{}", self.record.to_latex(), self.label)
    }
}
