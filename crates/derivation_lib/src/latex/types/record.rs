use super::super::LatexFmt;
use syntax::types::{Record, Type};

impl<Ty> LatexFmt for Record<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{ {} \\}}",
            self.records
                .iter()
                .map(|(lb, ty)| format!("{} = {}", lb, ty.to_latex()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
