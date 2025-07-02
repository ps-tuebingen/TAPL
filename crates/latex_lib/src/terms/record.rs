use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Record, Term};

impl<T> LatexFmt for Record<T>
where
    T: Term + LatexFmt,
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
