use super::super::LatexFmt;
use syntax::types::{Tuple, Type};

impl<Ty> LatexFmt for Tuple<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "({})",
            self.tys
                .iter()
                .map(|ty| ty.to_latex())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
