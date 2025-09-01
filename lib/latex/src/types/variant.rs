use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Variant};

impl<Lang> LatexFmt for Variant<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} \\rangle",
            self.variants
                .iter()
                .map(|(lb, ty)| format!("{} = {}", lb.to_latex(conf), ty.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
