use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Term, Variant},
    types::Type,
};

impl<T, Ty> LatexFmt for Variant<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\langle {} = {} \\rangle as {}",
            self.label,
            self.term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
