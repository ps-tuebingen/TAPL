use super::super::{LatexConfig, LatexFmt};
use syntax::values::{Pair, Value};

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
