use super::super::LatexFmt;
use syntax::types::{OpLambdaSub, Type};

impl<Ty> LatexFmt for OpLambdaSub<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\lambda {}<:{}.{}",
            self.var,
            self.sup.to_latex(),
            self.body.to_latex()
        )
    }
}
