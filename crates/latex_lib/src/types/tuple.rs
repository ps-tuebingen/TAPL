use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Tuple, Type};

impl<Ty> LatexFmt for Tuple<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "({})",
            self.tys
                .iter()
                .map(|ty| ty.to_latex(conf))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
