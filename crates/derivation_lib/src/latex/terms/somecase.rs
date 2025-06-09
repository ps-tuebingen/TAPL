use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{SomeCase, Term};

impl<T> LatexFmt for SomeCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "case {} of \\{{ Nothing \\Rightarrow {} \\mid Something({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.none_term.to_latex(conf),
            self.some_var,
            self.some_term.to_latex(conf)
        )
    }
}
