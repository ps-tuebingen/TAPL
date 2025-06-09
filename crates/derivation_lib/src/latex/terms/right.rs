use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Right<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "inl({}) as {}",
            self.right_term.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
