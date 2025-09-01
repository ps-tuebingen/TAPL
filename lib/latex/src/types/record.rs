use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, types::Record};

impl<Lang> LatexFmt for Record<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, ty)| format!("\\text{{{}}} = {}", lb.to_latex(conf), ty.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
