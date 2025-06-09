use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{App, Term};

impl<T> LatexFmt for App<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("{} {}", self.fun.to_latex(conf), self.arg.to_latex(conf))
    }
}
