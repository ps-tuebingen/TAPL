use super::super::LatexFmt;
use syntax::terms::{Term, Try};

impl<T> LatexFmt for Try<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "try \\{{ {} \\}} with \\{{ {} \\}}",
            self.term.to_latex(),
            self.handler.to_latex()
        )
    }
}
