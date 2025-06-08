use super::super::LatexFmt;
use eval::values::{Tuple, Value};

impl<V> LatexFmt for Tuple<V>
where
    V: Value + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!(
            "\\{{ {} \\}}",
            self.vals
                .iter()
                .map(|ty| ty.to_latex())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
