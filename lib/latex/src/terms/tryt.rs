use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Try};

impl<Lang> LatexFmt for Try<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{try}} \\{{ {} \\}} \\text{{with}} \\{{ {} \\}}",
            self.term.to_latex(conf),
            self.handler.to_latex(conf)
        )
    }
}
