use super::super::LatexFmt;
use syntax::terms::{Pair, Term};

impl<T> LatexFmt for Pair<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("\\{{ {}, {} \\}}", self.fst.to_latex(), self.snd.to_latex())
    }
}
