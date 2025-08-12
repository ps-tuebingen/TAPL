use super::super::{LatexConfig, LatexFmt};
use syntax::values::{Something, Value};

impl<V> LatexFmt for Something<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!("something({})", self.val.to_latex(conf))
    }
}
