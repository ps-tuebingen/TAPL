use super::super::LatexFmt;
use syntax::terms::{App, Term};

impl<T> LatexFmt for App<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("{} {}", self.fun.to_latex(), self.arg.to_latex())
    }
}
