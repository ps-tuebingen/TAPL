use super::super::LatexFmt;
use syntax::types::{OpApp, Type};

impl<Ty> LatexFmt for OpApp<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("({}[{}])", self.fun.to_latex(), self.arg.to_latex())
    }
}
