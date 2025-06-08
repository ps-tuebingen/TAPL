use super::super::LatexFmt;
use syntax::types::{Type, Variant};

impl<Ty> LatexFmt for Variant<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\langle {} \\rangle",
            self.variants
                .iter()
                .map(|(lb, ty)| format!("{} = {}", lb, ty.to_latex()))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
