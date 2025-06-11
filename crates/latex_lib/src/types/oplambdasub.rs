use super::super::{LatexConfig, LatexFmt};
use syntax::types::{OpLambdaSub, Type};

impl<Ty> LatexFmt for OpLambdaSub<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\lambda {}<:{}.{}",
            self.var,
            self.sup.to_latex(conf),
            self.body.to_latex(conf)
        )
    }
}
