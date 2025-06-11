use super::super::{LatexConfig, LatexFmt};
use syntax::{
    types::Type,
    values::{Fold, Value},
};

impl<V, Ty> LatexFmt for Fold<V, Ty>
where
    V: Value + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "fold[{}]({})",
            self.ty.to_latex(conf),
            self.val.to_latex(conf)
        )
    }
}
