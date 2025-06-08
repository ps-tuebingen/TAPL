use super::super::LatexFmt;
use syntax::terms::{Term, TryWithVal};

impl<T> LatexFmt for TryWithVal<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "try \\{{ {} \\}} catch \\{{ {} \\}}",
            self.term.to_latex(),
            self.handler.to_latex()
        )
    }
}
