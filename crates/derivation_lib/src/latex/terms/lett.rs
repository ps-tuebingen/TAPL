use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Let, Term};

impl<T> LatexFmt for Let<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "let ({} = {}) in {}",
            self.var,
            self.bound_term.to_latex(conf),
            self.in_term.to_latex(conf)
        )
    }
}
