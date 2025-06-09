use super::super::{LatexConfig, LatexFmt};
use syntax::{
    terms::{Cons, Term},
    types::Type,
};

impl<T, Ty> LatexFmt for Cons<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\text{{Cons}}[{}]({},{})",
            self.ty.to_latex(conf),
            self.head.to_latex(conf),
            self.tail.to_latex(conf)
        )
    }
}
