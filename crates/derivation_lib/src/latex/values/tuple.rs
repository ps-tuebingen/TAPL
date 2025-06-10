use super::super::{LatexConfig, LatexFmt};
use syntax::values::{Tuple, Value};

impl<V> LatexFmt for Tuple<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.vals
                .iter()
                .map(|ty| ty.to_latex(conf))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
