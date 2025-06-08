use super::super::LatexFmt;
use syntax::types::{OpLambda, Type};

impl<Ty> LatexFmt for OpLambda<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\lambda {}::{}.{}",
            self.var,
            self.annot.to_latex(),
            self.body.to_latex()
        )
    }
}
