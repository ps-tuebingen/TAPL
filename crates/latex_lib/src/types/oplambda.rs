use super::super::{LatexConfig, LatexFmt};
use syntax::types::{OpLambda, Type};

impl<Ty> LatexFmt for OpLambda<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}::{}.{}",
            self.var,
            self.annot.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
