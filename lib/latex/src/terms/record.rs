use super::super::{LatexConfig, LatexFmt};
use syntax::{language::Language, terms::Record};

impl<Lang> LatexFmt for Record<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, t)| format!("\\text{{{}}}={}", lb.to_latex(conf), t.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
