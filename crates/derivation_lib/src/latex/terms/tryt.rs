use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, Try};

impl<T> LatexFmt for Try<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "try \\{{ {} \\}} with \\{{ {} \\}}",
            self.term.to_latex(conf),
            self.handler.to_latex(conf)
        )
    }
}
