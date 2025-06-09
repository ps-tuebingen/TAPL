use super::super::{LatexConfig, LatexFmt};
use syntax::terms::{Term, Tuple};

impl<T> LatexFmt for Tuple<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        format!(
            "\\{{ {} \\}}",
            self.terms
                .iter()
                .map(|t| t.to_latex(conf))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
