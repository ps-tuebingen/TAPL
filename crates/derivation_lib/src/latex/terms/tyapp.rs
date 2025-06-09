use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Term, TyApp},
    types::Type,
};

impl<T, Ty> LatexFmt for TyApp<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "(({})[{}])",
            self.fun.to_latex(conf),
            self.arg.to_latex(conf)
        )
    }
}
