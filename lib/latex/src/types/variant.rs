use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Type, Variant};

impl<Ty> LatexFmt for Variant<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} \\rangle",
            self.variants
                .iter()
                .map(|(lb, ty)| format!("{} = {}", lb.to_latex(conf), ty.to_latex(conf)))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
