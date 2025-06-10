use super::super::{LatexConfig, LatexFmt};
use syntax::{
    types::Type,
    values::{Cons, Value},
};

impl<V, Ty> LatexFmt for Cons<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "Cons[{}]({},{})",
            self.ty.to_latex(conf),
            self.head.to_latex(conf),
            self.tail.to_latex(conf)
        )
    }
}
