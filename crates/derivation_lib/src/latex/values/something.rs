use super::super::LatexFmt;
use eval::values::{Something, Value};

impl<V> LatexFmt for Something<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("something({})", self.val.to_latex())
    }
}
