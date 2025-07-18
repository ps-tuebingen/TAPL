use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Record, Type};

impl<Ty> LatexFmt for Record<Ty>
where
    Ty: Type + LatexFmt,
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
