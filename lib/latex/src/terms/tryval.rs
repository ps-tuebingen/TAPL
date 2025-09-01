use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> LatexFmt for TryWithVal<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{try}} \\{{ {} \\}} \\text{{catch}} \\{{ {} \\}}",
            self.term.to_latex(conf),
            self.handler.to_latex(conf)
        )
    }
}
