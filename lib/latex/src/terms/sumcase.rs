use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::SumCase};

impl<Lang> LatexFmt for SumCase<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{case}} {} \\text{{of}} \\{{ \\text{{inl}}({}) \\Rightarrow {} \\mid \\text{{inr}}({}) \\Rightarrow {} \\}}",
            self.bound_term.to_latex(conf),
            self.left_var.to_latex(conf),
            self.left_term.to_latex(conf),
            self.right_var.to_latex(conf),
            self.right_term.to_latex(conf)
        )
    }
}
