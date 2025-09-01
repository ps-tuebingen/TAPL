use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Lambda};

impl<Lang> LatexFmt for Lambda<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}:{}.{}",
            self.var.to_latex(conf),
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
