use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::SomeCase};

impl<Lang> LatexFmt for SomeCase<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
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
