use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{SomeCase, Term};

impl<T> LatexFmt for SomeCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ \\text{{Nothing}} \\Rightarrow {} \\mid \\text{{Something}}({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.none_term.to_latex(conf),
            self.some_var.to_latex(conf),
            self.some_term.to_latex(conf)
        )
    }
}
