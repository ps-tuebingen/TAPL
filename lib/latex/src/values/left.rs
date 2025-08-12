use super::super::{LatexConfig, LatexFmt};
use syntax::{
    types::Type,
    values::{Left, Value},
};

impl<V, Ty> LatexFmt for Left<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "inl({}) as {}",
            self.left_val.to_latex(conf),
            self.ty.to_latex(conf)
        )
    }
}
