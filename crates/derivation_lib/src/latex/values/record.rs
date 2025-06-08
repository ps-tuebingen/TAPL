use super::super::LatexFmt;
use eval::values::{Record, Value};

impl<V> LatexFmt for Record<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, v)| format!("{}={}", lb, v.to_latex()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
