use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, values::Record};

impl<Lang> LatexFmt for Record<Lang>
where
    Lang: Language,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, v)| format!("{}={}", lb.to_latex(conf), v.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
