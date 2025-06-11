use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{SumCase, Term};

impl<T> LatexFmt for SumCase<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ \\text{{inl}}({}) \\Rightarrow {} \\mid \\text{{inr}}({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.left_var,
            self.left_term.to_latex(conf),
            self.right_var,
            self.right_term.to_latex(conf)
        )
    }
}
