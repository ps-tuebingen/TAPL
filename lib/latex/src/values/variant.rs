use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Variant};

impl<Lang> LatexFmt for Variant<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} = {} \\rangle",
            self.label.to_latex(conf),
            self.val.to_latex(conf)
        )
    }
}
