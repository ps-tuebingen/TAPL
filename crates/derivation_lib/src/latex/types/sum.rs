use super::super::LatexFmt;
use syntax::types::{Sum, Type};

impl<Ty> LatexFmt for Sum<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("({}+{})", self.left.to_latex(), self.right.to_latex())
    }
}
