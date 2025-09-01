use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Let};

impl<Lang> LatexFmt for Let<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{ let }} ({} = {}) \\text{{ in }} {}",
            self.var.to_latex(conf),
            self.bound_term.to_latex(conf),
            self.in_term.to_latex(conf)
        )
    }
}
