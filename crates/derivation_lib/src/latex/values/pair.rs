use super::super::LatexFmt;
use eval::values::{Pair, Value};

impl<V> LatexFmt for Pair<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("\\{{ {},{} \\}}", self.fst.to_latex(), self.snd.to_latex())
    }
}
