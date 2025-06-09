use super::super::{LatexConfig, LatexFmt};
use eval::values::{Record, Value};

impl<V> LatexFmt for Record<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, v)| format!("{}={}", lb, v.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
