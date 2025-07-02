use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Mu, Type};

impl<Ty> LatexFmt for Mu<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\mu {}. {}",
            self.var.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
