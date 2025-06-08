use super::super::LatexFmt;
use syntax::types::{Product, Type};

impl<Ty> LatexFmt for Product<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{ {} \\times {} \\}}",
            self.fst.to_latex(),
            self.snd.to_latex()
        )
    }
}
