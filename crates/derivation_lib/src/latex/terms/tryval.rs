use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, TryWithVal};

impl<T> LatexFmt for TryWithVal<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "try \\{{ {} \\}} catch \\{{ {} \\}}",
            self.term.to_latex(conf),
            self.handler.to_latex(conf)
        )
    }
}
