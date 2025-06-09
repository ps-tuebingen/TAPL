use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Exists, Type};

impl<Ty> LatexFmt for Exists<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{\\exists {} :: {}, {} \\}}",
            self.var,
            self.kind.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
