use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{If, Term};

impl<T> LatexFmt for If<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{ if }} ({}) \\{{ {} \\}} \\text{{ else }} \\{{ {} \\}}",
            self.if_cond.to_latex(conf),
            self.then_term.to_latex(conf),
            self.else_term.to_latex(conf)
        )
    }
}
