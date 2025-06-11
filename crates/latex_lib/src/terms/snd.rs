use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Snd, Term};

impl<T> LatexFmt for Snd<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("({}).1", self.term.to_latex(conf))
    }
}
