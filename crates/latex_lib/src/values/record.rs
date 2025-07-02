use super::super::{LatexConfig, LatexFmt};
use syntax::values::{Record, Value};

impl<V> LatexFmt for Record<V>
where
    V: Value + LatexFmt,
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
