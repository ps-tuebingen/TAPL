use super::super::{LatexConfig, LatexFmt};
use eval::values::{Pair, Value};

impl<V> LatexFmt for Pair<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {},{} \\}}",
            self.fst.to_latex(conf),
            self.snd.to_latex(conf)
        )
    }
}
