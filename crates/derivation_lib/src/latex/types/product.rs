use super::super::{LatexConfig, LatexFmt};
use syntax::types::{Product, Type};

impl<Ty> LatexFmt for Product<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\times {} \\}}",
            self.fst.to_latex(conf),
            self.snd.to_latex(conf)
        )
    }
}
