use super::super::LatexFmt;
use syntax::terms::{Record, Term};

impl<T> LatexFmt for Record<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{{}\\}}",
            self.records
                .iter()
                .map(|(lb, t)| format!("{}={}", lb, t.to_latex()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
