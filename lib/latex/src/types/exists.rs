use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Exists};

impl<Lang> LatexFmt for Exists<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{\\exists {} :: {}, {} \\}}",
            self.var.to_latex(conf),
            self.kind.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
