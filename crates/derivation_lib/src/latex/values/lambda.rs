use super::super::LatexFmt;
use eval::values::Lambda;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Lambda<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\lambda {}:{}.{}",
            self.var,
            self.annot.to_latex(),
            self.body.to_latex()
        )
    }
}
